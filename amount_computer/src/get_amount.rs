use crate::config;
use log::{info,warn};
use mysql_async::{Pool, Row};
use mysql_async::prelude::Queryable;
use chrono::prelude::*;
use chrono::{NaiveDateTime,Duration,DateTime};
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;


pub async fn get_day_amount() {

    //连接数据库
    let pool: Pool = config::get_db();
    let mut conn = pool.get_conn().await.unwrap();
    //读取配置文件
    let file = match File::open("./config/config.json") {
        Ok(f) => f,
        Err(_error) => {
            warn!("The configuration file does not exist:{:?}", "wallet_config.json");
            drop(conn);
	        pool.disconnect().await.unwrap();
            return ;
        }
    };
    let reader = BufReader::new(file);
    let value_name: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let low_app: Vec<serde_json::Value> = value_name["applevel"]["low"].as_array().unwrap().to_vec();
    let mid_app: Vec<serde_json::Value> = value_name["applevel"]["middle"].as_array().unwrap().to_vec();
    let high_app: Vec<serde_json::Value> = value_name["applevel"]["high"].as_array().unwrap().to_vec();
    let mut low_vec: Vec<String> = Vec::new();
    for v in low_app.iter() {
        let v_str: String = v.as_str().unwrap().to_string();
        low_vec.push(v_str);
    }
    let mut mid_vec: Vec<String> = Vec::new();
    for v in mid_app.iter() {
        let v_str: String = v.as_str().unwrap().to_string();
        mid_vec.push(v_str);
    }
    let mut high_vec: Vec<String> = Vec::new();
    for v in high_app.iter() {
        let v_str: String = v.as_str().unwrap().to_string();
        high_vec.push(v_str);
    }
    //以UTC时间为标准获取24的交易额度
    let time_hour:i64 = value_name["static_hour"].as_i64().unwrap();
    let utc: DateTime<Utc> = Utc::now();
    let naive_utc: NaiveDateTime = utc.naive_utc();
    let end_time: NaiveDateTime = naive_utc.checked_sub_signed(Duration::hours(time_hour)).unwrap();
    info!("Statistical time = {}--{}",end_time,naive_utc);
    //先查询当前24小时范围内是否有数据
    let trans_sql = format!("select amount, wallet_id,status from transfer where created > \'{}\' and created < \'{}\'",end_time,naive_utc);
    let row: Vec<Row> = conn.query(trans_sql).await.unwrap(); 
    if row.is_empty(){
        info!("Trans_APPID = None ");
    } 
    let withdraw_sql = format!("select withdraw_amount, wallet_id,status from withdraw where created > \'{}\' and created < \'{}\'",end_time,naive_utc);
    let with_row: Vec<Row> = conn.query(withdraw_sql).await.unwrap(); 
    if with_row.is_empty(){
        info!("WithDraw_APPID = None ");
    }
    if  row.is_empty() && with_row.is_empty(){
        drop(conn);
	    pool.disconnect().await.unwrap();
        return 
    }
    //交易统计计算
    //查找wallet id对应的app id
    if !row.is_empty(){
        let mut trans_stat_low = HashMap::<String,i64>::new();
        let mut trans_stat_mid = HashMap::<String,i64>::new();
        let mut trans_stat_high = HashMap::<String,i64>::new();
        let mut trans_stat_appid = HashMap::<String,i64>::new();
        for v in row.iter(){
            let w_id: String = v.get(1).unwrap();
            let status: String = v.get(2).unwrap();
            if status != String::from("succeeded"){
                continue;
            }
            let app_sql = format!("select appid from wallet where id = \'{}\'",w_id);
            let row_a: Vec<Row> = conn.query(app_sql).await.unwrap(); 
            if row_a.is_empty(){
                warn!("this wallet id is None:{}",w_id);
                drop(conn);
                pool.disconnect().await.unwrap();
                return 
            }
            let appid: String = row_a[0].get(0).unwrap();
            trans_stat_appid.insert(appid.clone(), 0);
            if low_vec.iter().any(|x| x ==&appid) {
                trans_stat_low.insert(w_id, 0);
            }else if mid_vec.iter().any(|x| x == &appid) {
                trans_stat_mid.insert(w_id, 0);
            }else if high_vec.iter().any(|x| x == &appid) {
                trans_stat_high.insert(w_id, 0);
            }else{
                warn!("in the config file this appid does not:{}",appid);
                drop(conn);
                pool.disconnect().await.unwrap();
                return
            }
        } 
        //transfer计算规则
        let t_low_percent:i64 = value_name["transfer"]["low"]["percent"].as_i64().unwrap();
        let t_low_min:i64 = value_name["transfer"]["low"]["min"].as_i64().unwrap();
        let t_low_max:i64 = value_name["transfer"]["low"]["max"].as_i64().unwrap();
        let t_middle_percent:i64 = value_name["transfer"]["middle"]["percent"].as_i64().unwrap();
        let t_middle_min:i64 = value_name["transfer"]["middle"]["min"].as_i64().unwrap();
        let t_middle_max:i64 = value_name["transfer"]["middle"]["max"].as_i64().unwrap();
        let t_high_percent:i64 = value_name["transfer"]["high"]["percent"].as_i64().unwrap();
        let t_high_min:i64 = value_name["transfer"]["high"]["min"].as_i64().unwrap();
        let t_high_max:i64 = value_name["transfer"]["high"]["max"].as_i64().unwrap();
        for v in row.iter() {
            let amount: i64 = v.get(0).unwrap();
            let status: String = v.get(2).unwrap();
            if status != String::from("succeeded"){
                continue;
            }
            let wallet_id: String = v.get(1).unwrap();
            if trans_stat_low.get(&wallet_id).is_some(){
                let tmp_value = trans_stat_low.clone();
                let value = tmp_value.get(&wallet_id).unwrap();
                let mut charge = amount* t_low_percent;
                //info!("amount = {}--value = {}--charge = {}",amount, value, charge);
                if charge < t_low_min {
                    charge = t_low_min;
                }else if charge > t_low_max{
                    charge = t_low_max;
                }
                let add_charge = value+charge;
                trans_stat_low.insert(wallet_id, add_charge);
            }else if trans_stat_mid.get(&wallet_id).is_some(){
                let tmp_value = trans_stat_mid.clone();
                let value = tmp_value.get(&wallet_id).unwrap();
                let mut charge = amount * t_middle_percent;
                if charge < t_middle_min {
                    charge = t_middle_min;
                }else if charge > t_middle_max{
                    charge = t_middle_max;
                }
                trans_stat_mid.insert(wallet_id, value+charge);
            } else if trans_stat_high.get(&wallet_id).is_some(){
                let tmp_value = trans_stat_high.clone();
                let value = tmp_value.get(&wallet_id).unwrap();
                let mut charge = amount * t_high_percent;
                if charge < t_high_min {
                    charge = t_high_min;
                }else if charge > t_high_max{
                    charge = t_high_max;
                }
                trans_stat_high.insert(wallet_id, value+charge);
            } 
        } 
        
        for (k,v) in trans_stat_low.iter(){
            let app_sql = format!("select appid from wallet where id = \'{}\'",k);
            let row_a: Vec<Row> = conn.query(app_sql).await.unwrap(); 
            if row_a.is_empty(){
                warn!("this wallet id is None:{}",k);
                drop(conn);
                pool.disconnect().await.unwrap();
                return 
            }
            let appid: String = row_a[0].get(0).unwrap();
            let tmp_app = trans_stat_appid.clone();
            let a_value = tmp_app.get(&appid).unwrap();
            trans_stat_appid.insert(appid, v+a_value);
        }
        for (k,v) in trans_stat_mid.iter(){
            let app_sql = format!("select appid from wallet where id = \'{}\'",k);
            let row_a: Vec<Row> = conn.query(app_sql).await.unwrap(); 
            if row_a.is_empty(){
                warn!("this wallet id is None:{}",k);
                drop(conn);
                pool.disconnect().await.unwrap();
                return 
            }
            let appid: String = row_a[0].get(0).unwrap();
            let tmp_app = trans_stat_appid.clone();
            let a_value = tmp_app.get(&appid).unwrap();
            trans_stat_appid.insert(appid, v+a_value);
        }
        for (k,v) in trans_stat_high.iter(){
            let app_sql = format!("select appid from wallet where id = \'{}\'",k);
            let row_a: Vec<Row> = conn.query(app_sql).await.unwrap(); 
            if row_a.is_empty(){
                warn!("this wallet id is None:{}",k);
                drop(conn);
                pool.disconnect().await.unwrap();
                return 
            }
            let appid: String = row_a[0].get(0).unwrap();
            let tmp_app = trans_stat_appid.clone();
            let a_value = tmp_app.get(&appid).unwrap();
            trans_stat_appid.insert(appid, v+a_value);
        }
        info!("Trans_APPID = {:?}",trans_stat_appid);
    }
    //退款统计计算
    //查找wallet id对应的app id
    if !with_row.is_empty() {
        let mut with_stat_low = HashMap::<String,i64>::new();
        let mut with_stat_mid = HashMap::<String,i64>::new();
        let mut with_stat_high = HashMap::<String,i64>::new();
        let mut with_stat_appid = HashMap::<String,i64>::new();
        for v in with_row.iter(){
            let w_id: String = v.get(1).unwrap();
            let status: String = v.get(2).unwrap();
            if status != String::from("succeeded"){
                continue;
            }
            let app_sql = format!("select appid from wallet where id = \'{}\'",w_id);
            let row_a: Vec<Row> = conn.query(app_sql).await.unwrap(); 
            if row_a.is_empty(){
                warn!("this wallet id is None:{}",w_id);
                drop(conn);
                pool.disconnect().await.unwrap();
                return 
            }
            let appid: String = row_a[0].get(0).unwrap();
            with_stat_appid.insert(appid.clone(), 0);
            if low_vec.iter().any(|x| x ==&appid) {
                with_stat_low.insert(w_id, 0);
            }else if mid_vec.iter().any(|x| x == &appid) {
                with_stat_mid.insert(w_id, 0);
            }else if high_vec.iter().any(|x| x == &appid) {
                with_stat_high.insert(w_id, 0);
            }else{
                warn!("in the config file this appid does not:{}",appid);
                drop(conn);
                pool.disconnect().await.unwrap();
                return
            }
        } 
        //transfer计算规则
        let w_low_percent:i64 = value_name["withdraw"]["low"]["percent"].as_i64().unwrap();
        let w_middle_percent1:i64 = value_name["withdraw"]["middle"]["limit_min"].as_i64().unwrap();
        let w_middle_percent2:i64 = value_name["withdraw"]["middle"]["limit_max"].as_i64().unwrap();
        let w_middle_limit:i64 = value_name["withdraw"]["middle"]["limit_value"].as_i64().unwrap();
        let w_high_percent1:i64 = value_name["withdraw"]["high"]["limit_min"].as_i64().unwrap();
        let w_high_percent2:i64 = value_name["withdraw"]["high"]["limit_max"].as_i64().unwrap();
        let w_high_limit:i64 = value_name["withdraw"]["high"]["limit_value"].as_i64().unwrap();
        for v in with_row.iter() {
            let amount: i64 = v.get(0).unwrap();
            let wallet_id: String = v.get(1).unwrap();
            let status: String = v.get(2).unwrap();
            if status != String::from("succeeded"){
                continue;
            }
            if with_stat_low.get(&wallet_id).is_some(){
                let test_vec = with_stat_low.clone();
                let value = test_vec.get(&wallet_id).unwrap();
                let charge = amount * w_low_percent;
                with_stat_low.insert(wallet_id, value+charge);
            }else if with_stat_mid.get(&wallet_id).is_some(){
                let test_vec = with_stat_mid.clone();
                let value = test_vec.get(&wallet_id).unwrap();
                let mut charge = 0;
                if (amount*1000) > w_middle_limit{
                    charge *= w_middle_percent2;
                }else{
                    charge *= w_middle_percent1;
                }
                with_stat_mid.insert(wallet_id, value+charge);
            } else if with_stat_high.get(&wallet_id).is_some(){
                let test_vec = with_stat_high.clone();
                let value = test_vec.get(&wallet_id).unwrap();
                let mut charge = 0;
                if amount > w_high_limit{
                    charge *= w_high_percent2;
                }else{
                    charge *= w_high_percent1;
                }
                with_stat_high.insert(wallet_id, value+charge);
            } 
        } 
        for (k,v) in with_stat_low.iter(){
            let app_sql = format!("select appid from wallet where id = \'{}\'",k);
            let row_a: Vec<Row> = conn.query(app_sql).await.unwrap(); 
            if row_a.is_empty(){
                warn!("this wallet id is None:{}",k);
                drop(conn);
                pool.disconnect().await.unwrap();
                return 
            }
            let appid: String = row_a[0].get(0).unwrap();
            let tmp_app = with_stat_appid.clone();
            let a_value = tmp_app.get(&appid).unwrap();
            with_stat_appid.insert(appid, v+a_value);
        }
        for (k,v) in with_stat_mid.iter(){
            let app_sql = format!("select appid from wallet where id = \'{}\'",k);
            let row_a: Vec<Row> = conn.query(app_sql).await.unwrap(); 
            if row_a.is_empty(){
                warn!("this wallet id is None:{}",k);
                drop(conn);
                pool.disconnect().await.unwrap();
                return 
            }
            let appid: String = row_a[0].get(0).unwrap();
            let tmp_app = with_stat_appid.clone();
            let a_value = tmp_app.get(&appid).unwrap();
            with_stat_appid.insert(appid, v+a_value);
        }
        for (k,v) in with_stat_high.iter(){
            let app_sql = format!("select appid from wallet where id = \'{}\'",k);
            let row_a: Vec<Row> = conn.query(app_sql).await.unwrap(); 
            if row_a.is_empty(){
                warn!("this wallet id is None:{}",k);
                drop(conn);
                pool.disconnect().await.unwrap();
                return 
            }
            let appid: String = row_a[0].get(0).unwrap();
            let tmp_app = with_stat_appid.clone();
            let a_value = tmp_app.get(&appid).unwrap();
            with_stat_appid.insert(appid, v+a_value);
        }
        drop(conn);
        pool.disconnect().await.unwrap();
        info!("WithDraw_APPID = {:?}",with_stat_appid);
    }
} 
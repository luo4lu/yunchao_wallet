//use log::Level;
use log:: {info,warn};
use serde::Serialize;
use mysql_async::{Row, Pool};
use mysql_async::prelude::Queryable;
use redis::AsyncCommands;
use reqwest::Client;
use std::fs::File;
use std::io::BufReader;
use futures_util::StreamExt as _;
use sodiumoxide::crypto::box_;
use base64;
mod config;
//webhook密文数据通知类型
#[derive(Debug, Serialize)]
pub struct AuthReqwest{
    version: u32,          
    payload: String,     
    nonce: String        
}

#[tokio::main]
async fn main() {
    env_logger::init();
    //simple_logger::init_with_level(Level::Info).unwrap();
    //读取配置文件设置kafka初始状态
    let file = match File::open("./config/config_file.json") {
        Ok(f) => f,
        Err(_error) => {
            warn!("The configuration file does not exist:{:?}", "wallet_config.json");
            return ;
        }
    };
    let reader = BufReader::new(file);
    let value_name: serde_json::Value = serde_json::from_reader(reader).unwrap();
    //连接Redis数据库
    let redis: &str = value_name["redis_addr"].as_str().unwrap();
    let redis_path = format!("redis://{}/",redis);
    let redis_client = redis::Client::open(redis_path).unwrap();
    
    let mut pubsub_conn = redis_client.get_async_connection().await.unwrap().into_pubsub();
    let mut conn = redis_client.get_async_connection().await.unwrap();
    pubsub_conn.subscribe("__keyevent@0__:expired").await.unwrap();
    loop {
        let mut pubsub_stream = pubsub_conn.on_message();
        let pubsub_msg: String = pubsub_stream.next().await.unwrap().get_payload().unwrap();

        if 15 > pubsub_msg.len() {
            warn!("this redis key not need listen:{}",pubsub_msg);
            continue;
        }
        //读取文件获取下一次超时重传键
        let head_str1 = &pubsub_msg[0..14];
        if head_str1 != "webhook-expire" {
            warn!("this redis key not need listen:{}",pubsub_msg);
            continue;
        }
        let number = &pubsub_msg[pubsub_msg.len()-1..];
        let y = number.parse::<u32>().unwrap() + 1;
        let s = y.to_string();
        let object_id = &pubsub_msg[15..pubsub_msg.len()-2];
        let redis_key2 = format!("{}-{}",String::from("webhook-context"),object_id);
        let result: String = match conn.get(redis_key2.clone()).await{
            Ok(v) => v,
            Err(error) => {
                warn!("Error:{:?}",error);
                continue;
            }
        };
        //结果反序列
        let mut wallet_id = object_id;
        let result_json: serde_json::Value = serde_json::from_str(&result).unwrap();
        let event: String = result_json["event"].as_str().unwrap().to_string(); 
        if !(event == String::from("wallet.failed") || event == String::from("wallet.succeeded")){
            info!("current event = {}",event);
            wallet_id = result_json["data"]["wallet_id"]["id"].as_str().unwrap();
        }
        info!("Recv data result == {:?}====object_id == {}",result_json, object_id);
        //数据库连接 获取webhook推送地址
        let pool: Pool = config::get_db();
        let mut conn_mysql = pool.get_conn().await.unwrap();
        let sql_str1 = format!("select appid from wallet where id = \'{}\'",wallet_id);
        let row1: Vec<Row> = conn_mysql.query(sql_str1).await.unwrap();
        if row1.is_empty(){
            warn!("wallet select failed！！");
            continue;
        }
        let app_id: String = row1[0].get(0).unwrap();
        let sql_str = format!("select web_url from user_info where appid = \'{}\'",app_id);
        let row: Vec<Row> = conn_mysql.query(sql_str).await.unwrap();
        if row.is_empty(){
            warn!("user_info select failed！！");
            continue;
        }
        //释放资源
        drop(conn_mysql);
        pool.disconnect().await.unwrap();
        let web_url: String = row[0].get(0).unwrap();
        //数据库连接 查询平台秘钥与用户公钥
        let pool2: Pool = config::get_db2();
        let mut conn2 = pool2.get_conn().await.unwrap();
        let sql_str2 = format!("select pkc, root_index from consumer_v2 where api_key = \'{}\'",app_id);
        let row2: Vec<Row> = conn2.query(sql_str2).await.unwrap();
        if row2.is_empty(){
            info!("secret consumer_v2 select failed！！");
            continue;
        }
        let user_pkc: String = row2[0].get(0).unwrap();
        let root_index: i64 = row2[0].get(1).unwrap();
        let sql_str3 = format!("select sk0 from consumer_v2 where id = {}",root_index);
        let row3: Vec<Row> = conn2.query(sql_str3).await.unwrap();
        if row3.is_empty(){
            info!("secret consumer_v2 select failed！！");
            continue;
        }
        let root_sk0: String = row3[0].get(0).unwrap();
        //释放资源
        drop(conn2);
        match pool2.disconnect().await{
            Ok(_) => {
                info!("pool resource delete success!!");
            }
            Err(error) => {
                warn!("pool resource delete failed!!{:?}",error);
            }
        };

        let send_params = serde_json::to_vec(&result_json).unwrap();
        let nonce = box_::gen_nonce();
        let pk = box_::PublicKey::from_slice(&base64::decode(user_pkc).unwrap()).unwrap();
        let sk = box_::SecretKey::from_slice(&base64::decode(root_sk0).unwrap()).unwrap();
        let their_precomputed_key = box_::precompute(&pk, &sk);
        let ciphertext = box_::seal_precomputed(&send_params, &nonce, &their_precomputed_key);
        let ciphertext_str = base64::encode(ciphertext);
        let nonce_str = base64::encode(nonce);
        let web_params = AuthReqwest{
            version: 2,          
            payload: ciphertext_str,     
            nonce:nonce_str   
        };
        //启动一个reqwest客户端连接句柄
        info!("start request malloc client");
        let info_client = Client::new();
        info!("end request malloc client");
        let request_info = match info_client
        .post(&web_url)
        .json(&web_params)
        .send()
        .await{
            Ok(v)=>v,
            Err(error)=>{
                warn!("url={} addr analysis error:{:?}",web_url,error);
                //重新写入redis
                let file_r = match File::open("./config/retrans_conf.json") {
                    Ok(f) => f,
                    Err(_error) => {
                        warn!("The configuration file does not exist:{:?}", "retrans_conf.json");
                        return ;
                    }
                };
                let reader_r = BufReader::new(file_r);
                let value_r: serde_json::Value = serde_json::from_reader(reader_r).unwrap();
                let at_time: usize = match value_r[s.clone()].as_u64(){
                    Some(t) => t as usize,
                    None => {
                        let _:() = conn.del(redis_key2).await.unwrap();
                        continue;
                    }
                };
                let redis_key1 = format!("{}-{}-{}",head_str1,object_id,s);
                info!("Retrans-key ===== {}",redis_key1);
                let _: () = conn.set(redis_key1.clone(),s).await.unwrap();
                let _: () = conn.expire(redis_key1,at_time).await.unwrap();
                continue;
            }
        };
        let code_status = request_info.status().as_u16();
        if code_status != 200 {
            //重新写入redis
            let file_r = match File::open("./config/retrans_conf.json") {
                Ok(f) => f,
                Err(_error) => {
                    warn!("The configuration file does not exist:{:?}", "retrans_conf.json");
                    return ;
                }
            };
            let reader_r = BufReader::new(file_r);
            let value_r: serde_json::Value = serde_json::from_reader(reader_r).unwrap();
            let at_time: usize = match value_r[s.clone()].as_u64(){
                Some(t) => t as usize,
                None => {
                    let _:() = conn.del(redis_key2).await.unwrap();
                    continue;
                }
            };
            let redis_key1 = format!("{}-{}-{}",head_str1,object_id,s);
            info!("Retrans-key ===== {}",redis_key1);
            let _: () = conn.set(redis_key1.clone(),s).await.unwrap();
            let _: () = conn.expire(redis_key1,at_time).await.unwrap();
            continue;
        }
        let _:() = conn.del(redis_key2).await.unwrap();
    }
}
use serde::{Deserialize,Serialize};
use log::Level;
use log:: warn;
use clap::ArgMatches;
use mysql_async::{Row, Pool};
use mysql_async::prelude::Queryable;
use redis::AsyncCommands;
use chrono::{NaiveDateTime};
use reqwest::Client;
use std::fs::File;
use std::io::BufReader;
use futures_util::StreamExt as _;
use std::time;

mod config;


#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Warn).unwrap();
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
        let number = &pubsub_msg[pubsub_msg.len()-1..];
        let y = number.parse::<u32>().unwrap() + 1;
        let s = y.to_string();
        //读取文件获取下一次超时重传键
        let head_str1 = &pubsub_msg[0..14];
        println!("HEAD = {}",head_str1);
        if head_str1 != "webhook-expire" {
            warn!("this redis key not need listen:{}",pubsub_msg);
            continue;
        }
        let object_id = &pubsub_msg[15..pubsub_msg.len()-2];
        let redis_key2 = format!("{}-{}",String::from("webhook-context"),object_id);
        println!("key2={}",redis_key2);
        let result: String = match conn.get(redis_key2.clone()).await{
            Ok(v) => v,
            Err(error) => {
                warn!("Error:{:?}",error);
                continue;
            }
        };
        //结果反序列
        let result_json: serde_json::Value = serde_json::from_str(&result).unwrap();
        let app_id: String = result_json["wallet_id"]["appid"].as_str().unwrap().to_string();
        //数据库连接 获取webhook推送地址
        let pool: Pool = config::get_db();
        let mut conn_mysql = pool.get_conn().await.unwrap();
        let sql_str = format!("select web_url from user_info where appid = \'{}\'",app_id);
        let row: Vec<Row> = conn_mysql.query(sql_str).await.unwrap();
        if row.is_empty(){
            warn!("select failed！！");
            continue;
        }
        //释放资源
        drop(conn_mysql);
        pool.disconnect().await.unwrap();
        let web_url: String = row[0].get(0).unwrap();
        let info_client = Client::new();
               
        let request_info = info_client
        .post(&web_url)
        .json(&result_json)
        .send()
        .await
        .unwrap();
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
            let at_time: usize = value_r[s.clone()].as_u64().unwrap() as usize;
            let redis_key1 = format!("{}-{}-{}",head_str1,object_id,s);
            let _: () = conn.set(redis_key1.clone(),s).await.unwrap();
            let _: () = conn.expire(redis_key1,at_time).await.unwrap();
            continue;
        }
        let _:() = conn.del(redis_key2).await.unwrap();
    }
}
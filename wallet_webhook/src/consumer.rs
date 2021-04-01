use crate::config;
//use crate::data_struct::{RechargeServer, WithdrawServer, TransferServer};
use std::fs::File;
use std::io::BufReader;
use futures::StreamExt;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use mysql_async::{Row, Pool};
use mysql_async::prelude::Queryable;
use redis::AsyncCommands;

//kafka监听消息结构
#[derive(Deserialize,Serialize, Debug)]
pub struct CallbackInfo{
    data: serde_json::Value,
    rsp: serde_json::Value,
    code: i32
}

//webhook数据通知类型
#[derive(Debug, Serialize)]
pub struct WebhookReqwest {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    created: i64,
    event: String,
    data: serde_json::Value
}
struct CustomContext;

impl ClientContext for CustomContext {}
impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance){
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets {:?}", result);
    }
}

type LoggingConsumer = StreamConsumer<CustomContext>;

pub async fn consumer_server()
{
    //配置事件信息
    let wallet_create = String::from("wallet.create");
    let wallet_rst_pwd = String::from("wallet.rst_pwd");
    let settle_create = String::from("settle.create");
    let settle_confirm = String::from("settle.confirm");
    let settle_remove = String::from("settle.remove");
    let recharge = String::from("recharge.create");
    let withdraw = String::from("withdraw.create");
    let transfer = String::from("transfer.create");
    //读取配置文件设置kafka初始状态
    let file = match File::open("./config/wallet_config_file.json") {
        Ok(f) => f,
        Err(_error) => {
            warn!("The configuration file does not exist:{:?}", "wallet_config.json");
            return ;
        }
    };
    let reader = BufReader::new(file);
    let value_name: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let group_id: &str = value_name["group.id"].as_str().unwrap();
    let session_timeout: &str = value_name["session.timeout.ms"].as_str().unwrap();
    let auto_commit: &str = value_name["enable.auto.commit"].as_str().unwrap();
    let partition_eof: &str = value_name["enable.partition.eof"].as_str().unwrap();
    let brokers: &str = value_name["kafka_server"].as_str().unwrap();
    let topic_value: Vec<serde_json::Value> = value_name["kfk_topic"].as_array().unwrap().to_vec();
    let mut topic_vec: Vec<&str> = Vec::new();
    for v in topic_value.iter() {
        let value_str: &str = v.as_str().unwrap();
        topic_vec.push(value_str);
    }
    let context = CustomContext;
    //写入一个Redis超时事件
    let redis: &str = value_name["redis_addr"].as_str().unwrap();
    let redis_path = format!("redis://{}/",redis);
    let redis_client = redis::Client::open(redis_path).unwrap();

    let mut con = match redis_client.get_async_connection().await{
        Ok(c)=> c,
        Err(error) => {
            warn!("connect redis server failed:{:?}",error);
            return;
        }
    };
    //启动一个reqwest客户端连接句柄
    info!("start request malloc client");
    let info_client = reqwest::Client::new();
    info!("end request malloc client");

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", partition_eof) 
        .set("session.timeout.ms", session_timeout)
        .set("enable.auto.commit", auto_commit)
        .set("auto.commit.interval.ms", "5000")
        .set("allow.auto.create.topics", "true")
        .set("enable.auto.offset.store", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer.subscribe(&topic_vec).expect("Can't subscribe to specified topics");

    //开始监听指定topic事件
    let mut message_stream = consumer.start();
    warn!("启动消费");
    while let Some(message) = message_stream.next().await{
        match message {
            Err(e) => info!("this topic not neet: {}", e),
            Ok(m) => {
                let recv_event: String = String::from_utf8(m.key().unwrap().to_vec()).unwrap();
                let recv_topic: &str = m.topic();
                info!("recv_event={:?}---{}",recv_event,recv_topic);
                let payload_str = match m.payload_view::<str>(){
                    None => {
                        warn!("MessageInfo return null value");
                        continue;
                    },
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        continue;
                    }
                };
                let payload: CallbackInfo = match serde_json::from_str(&payload_str){
                    Ok(value) => value,
                    Err(error) => {
                        info!("this message not neet,get next message:{}.",error.to_string());
                        continue;
                    }
                };
                let object_data = payload.data;
                info!("Payload == {:?}",object_data);
                let code = payload.code;
                info!("Payload CODE ==={}",code);
                let mut _send_event: String = String::new();
                if 0 == code {
                    if recharge == recv_event{
                        _send_event = String::from("charge.succeeded");
                    }else if withdraw == recv_event{
                        _send_event = String::from("withdraw.succeeded");
                    }else if transfer == recv_event{
                        _send_event = String::from("transfer.succeeded");
                    }else if wallet_create ==recv_event || wallet_rst_pwd==recv_event {
                        _send_event = String::from("wallet.succeeded");
                    }else if settle_create==recv_event || settle_confirm==recv_event || settle_remove==recv_event{
                        _send_event = String::from("settle.succeeded");
                    }else{
                        info!("{:?} this key value not need,into get next message.",recv_event);
                        continue;
                    }
                }else{
                    //系统服务没有正确响应状态，无需发送到商户服务
                    if recharge == recv_event{
                        _send_event = String::from("charge.failed");
                    }else if withdraw == recv_event{
                        _send_event = String::from("withdraw.failed");
                    }else if transfer == recv_event{
                        _send_event = String::from("transfer.failed");
                    }else if  wallet_create ==recv_event || wallet_rst_pwd==recv_event {
                        _send_event = String::from("wallet.failed");
                    }else if settle_create==recv_event || settle_confirm==recv_event || settle_remove==recv_event{
                        _send_event = String::from("settle.failed");
                    }else{
                        info!("{:?} this key value not need,into get next message.",recv_event);
                        continue;
                    }
                }
                //获取数据格式
                let mut _app_id = String::new();
                let mut _object_id = String::new();   
                if wallet_create ==recv_event || wallet_rst_pwd==recv_event {
                    _app_id = object_data["appid"].as_str().unwrap().to_string();
                    _object_id = object_data["id"].as_str().unwrap().to_string();
                    info!("Rcev event is wallet object: appid=={}---id=={}",_app_id,_object_id);
                }else{
                    _app_id = object_data["wallet_id"]["appid"].as_str().unwrap().to_string();
                    _object_id = object_data["id"].as_str().unwrap().to_string();
                    info!("Rcev event is other object: appid=={}---id=={}",_app_id,_object_id);
                }
                //数据库连接
                info!("connect mysql data base!!!!!!!!!!!!!!!!!");
                let pool: Pool = config::get_db();
                let mut conn = pool.get_conn().await.unwrap();
                let sql_str = format!("select web_url, create_time from user_info where appid = \'{}\'",_app_id);
                let row: Vec<Row> = conn.query(sql_str).await.unwrap();
                if row.is_empty(){
                    info!("select failed！！");
                    continue;
                }
                //释放资源
                drop(conn);
                match pool.disconnect().await{
                    Ok(_) => {
                        info!("pool resource delete success!!");
                    }
                    Err(error) => {
                        warn!("pool resource delete failed!!{:?}",error);
                    }
                };
                let web_url: String = match row[0].get(0){
                    Some(v) => v,
                    None => {
                        warn!("row get 0 value is none");
                        String::from("None")
                    }
                };
                let create_time: Option<NaiveDateTime> = match row[0].get(1){
                    Some(v) => v,
                    None => {
                        warn!("row get 1 value is none");
                        continue;
                    }
                };
                let created: i64 = create_time.unwrap().timestamp();
                let params: WebhookReqwest = WebhookReqwest{
                    id: _object_id.clone(),
                    event_type: String::from("event"),
                    created,
                    event: _send_event,
                    data: object_data
                };
                info!("send object data to webhook!!!!!");
                let request_info = info_client
                .post(&web_url)
                .json(&params)
                .send()
                .await
                .unwrap();
                //提交消费通知
                consumer.commit_message(&m, CommitMode::Async).unwrap();
                let code_status = request_info.status().as_u16();
                info!("data commit success,code_status = {:?}",code_status);
                if code_status != 200 {
                     info!("webhook reqwest failed,into write retrans listen!!");
                    //Redis key
                    let redis_key1 = format!("webhook-expire-{}-0",_object_id);
                    let redis_key2 = format!("webhook-context-{}",_object_id);
                    let object_str: String = serde_json::to_string(&params).unwrap();
                    let _: () = con.set(redis_key1.clone(),1).await.unwrap();
                    let _: () = con.expire(redis_key1,5).await.unwrap();
                    let _: () = con.set(redis_key2.clone(),object_str).await.unwrap();
                }
            }   
        };
    }
}
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
use sodiumoxide::crypto::box_;
use base64;

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
    create: i64,
    event: String,
    data: serde_json::Value
}
//webhook密文数据通知类型
#[derive(Debug, Serialize)]
pub struct AuthReqwest{
    version: u32,          
    payload: String,     
    nonce: String        
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
    let wallet_bk = String::from("wallet.bk_query");
    let wallet_remove = String::from("wallet.remove");
    let settle_create = String::from("settle.create");
    let settle_confirm = String::from("settle.confirm");
    let settle_remove = String::from("settle.remove");
    let recharge = String::from("recharge.create");
    let withdraw = String::from("withdraw.create");
    let transfer = String::from("transfer.create");
    let transfer_order = String::from("transfer.order");
    let transfer_pay = String::from("transfer.pay");
    let transfer_refund = String::from("transfer.refund");
    let payment_create = String::from("payment.create");
    let payment_refund = String::from("payment.refund");
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
                let mut object_data = payload.data;
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
                    }else if transfer_order == recv_event || payment_create == recv_event{
                        _send_event = String::from("order.create");
                    }else if transfer_pay == recv_event{
                        _send_event = String::from("order.payed");
                    }else if transfer_refund == recv_event{
                        _send_event = String::from("order.refund");
                    }else if wallet_create ==recv_event || wallet_rst_pwd==recv_event || wallet_bk == recv_event || wallet_remove == recv_event {
                        _send_event = String::from("wallet.succeeded");
                    }else if settle_create==recv_event || settle_confirm==recv_event || settle_remove==recv_event{
                        _send_event = String::from("settle.succeeded");
                    }else if payment_refund == recv_event{
                        _send_event = String::from("refund.succeeded");
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
                    }else if transfer_order == recv_event{
                        _send_event = String::from("order.create");
                    }else if transfer_pay == recv_event || payment_create == recv_event{
                        _send_event = String::from("order.payed");
                    }else if transfer_refund == recv_event{
                        _send_event = String::from("order.refund");
                    }else if  wallet_create ==recv_event || wallet_rst_pwd==recv_event || wallet_bk == recv_event || wallet_remove == recv_event{
                        _send_event = String::from("wallet.failed");
                    }else if settle_create==recv_event || settle_confirm==recv_event || settle_remove==recv_event{
                        _send_event = String::from("settle.failed");
                    }else if payment_refund == recv_event{
                        _send_event = String::from("refund.failed");
                    }else{
                        info!("{:?} this key value not need,into get next message.",recv_event);
                        continue;
                    }
                }
                //获取数据格式
                let mut _app_id = String::new();
                let mut _object_id = String::new();   
                if wallet_create ==recv_event {
                    _app_id = match object_data["appid"].as_str(){
                        Some(v) => v.to_string(),
                        None => {
                            warn!("1.get appid is None!");
                            consumer.commit_message(&m, CommitMode::Async).unwrap();
                            continue
                        }
                    };
                    _object_id = match object_data["id"].as_str(){
                        Some(v) => v.to_string(),
                        None => {
                            warn!("2.get appid is None!");
                            consumer.commit_message(&m, CommitMode::Async).unwrap();
                            continue
                        }
                    };
                    info!("Rcev event is wallet object: appid=={}---id=={}",_app_id,_object_id);
                }else{
                    _app_id = match object_data["wallet_id"]["appid"].as_str(){
                        Some(v) => v.to_string(),
                        None => {
                            warn!("3.get appid is None!");
                            consumer.commit_message(&m, CommitMode::Async).unwrap();
                            continue
                        }
                    };
                    _object_id = match object_data["id"].as_str(){
                        Some(v) => v.to_string(),
                        None => {
                            warn!("4.get appid is None!");
                            consumer.commit_message(&m, CommitMode::Async).unwrap();
                            continue
                        }
                    };
                    info!("Rcev event is other object: appid=={}---id=={}",_app_id,_object_id);
                }
                //数据库连接
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
                let create: i64 = create_time.unwrap().timestamp();
                if _send_event == String::from("order.create") || _send_event == String::from("order.payed") || _send_event == String::from("order.refund") {
                    object_data = transfer_data(_send_event.clone(),object_data,recv_event).await;
                } 
                let params: WebhookReqwest = WebhookReqwest{
                    id: _object_id.clone(),
                    event_type: String::from("event"),
                    create,
                    event: _send_event,
                    data: object_data
                };
                info!("Send Params: {:?}",params);
                //数据库连接 查询平台秘钥与用户公钥
                let pool2: Pool = config::get_db2();
                let mut conn2 = pool2.get_conn().await.unwrap();
                let sql_str2 = format!("select pkc, root_index from consumer_v2 where api_key = \'{}\'",_app_id);
                let row2: Vec<Row> = conn2.query(sql_str2).await.unwrap();
                if row2.is_empty(){
                    info!("secret consumer_v2 select failed！！");
                    continue;
                }
                let user_pkc: String = row2[0].get(0).unwrap();
                let root_index: i64 = row2[0].get(1).unwrap();
                let sql_str3 = format!("select sk0 from root_v2 where id = {}",root_index);
                let row3: Vec<Row> = conn2.query(sql_str3).await.unwrap();
                if row3.is_empty(){
                    info!("secret root_v2 select failed！！");
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
                info!("public={}\nSK={}",user_pkc,root_sk0);
                let send_params = serde_json::to_vec(&params).unwrap();
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
                let info_client = reqwest::Client::new();
                info!("end request malloc client");
                let request_info = info_client
                .post(&web_url)
                .json(&web_params)
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


/*交易数据封装*/
#[derive(Debug, Serialize)]
pub struct OederCreate {
    channel: String,
    wallet_id: String, // 支付来源钱包
    amount: u64, // 支付来源钱包
    openid: Option<String>,  // 微信支付中存在此字段
    beans: Option<u64>,  // 志愿豆
    order: Option<String>, //志愿者平台传递的订单号
    #[serde(rename = "tradeType")]
    tradet_type: Option<String>, //志愿者平台传递的订单类型
    order_id: Option<String> // 订单id号
}
#[derive(Debug, Serialize)]
pub struct OederPayed{
    order_id: Option<String> // 订单id号
}
#[derive(Debug, Serialize)]
pub struct OederRefund{
    order_id: Option<String>, // 订单id号
    refund_amount: Option<u64>, // 退款金额
    refund_id:Option<String> //退款id
}
pub async fn transfer_data(event: String, result: serde_json::Value, recv_event: String) -> serde_json::Value {
    if event == String::from("order.create"){
        let mut channel: String = String::from("yunchaoplus");
        if recv_event == String::from("payment.create") {
            channel = String::from("wechat");
        }
        let wallet_id: String = result["wallet_id"]["id"].as_str().unwrap().to_string();
        let amount: u64 = result["amount"].as_u64().unwrap();
        let openid: Option<String> = match result["extra"]["sub_open_id"].as_str(){
            Some(v)=>Some(v.to_string()),
            None => None
        };
        let beans: Option<u64> = match result["extra"]["ai_zhi_yuan_extra"]["beans"].as_u64(){
            Some(v)=>Some(v),
            None => None
        };
        let order: Option<String> = match result["extra"]["ai_zhi_yuan_extra"]["orderId"].as_str(){
            Some(v)=>Some(v.to_string()),
            None => None
        };
        let tradet_type: Option<String> = match result["extra"]["ai_zhi_yuan_extra"]["tradeType"].as_str(){
            Some(v)=>Some(v.to_string()),
            None => None
        };
        let order_id: Option<String> = match result["id"].as_str(){
            Some(v)=>Some(v.to_string()),
            None => None
        };
        let param: OederCreate = OederCreate{
            channel,
            wallet_id, // 支付来源钱包
            amount, // 支付来源钱包
            openid,  // 微信支付中存在此字段
            beans,  // 志愿豆
            order, //志愿者平台传递的订单号
            tradet_type, //志愿者平台传递的订单类型
            order_id
        };
        return serde_json::to_value(param).unwrap();
    }else if event == String::from("order.payed"){
        let order_id: Option<String> = match result["transfer_order"]["id"].as_str(){
            Some(v)=>Some(v.to_string()),
            None => None
        };
        let param: OederPayed = OederPayed{
            order_id
        };
        return serde_json::to_value(param).unwrap();
    }else if event == String::from("order.refund"){
        let order_id: Option<String> = match result["transfer_order"]["id"].as_str(){
            Some(v)=>Some(v.to_string()),
            None => None
        };
        let refund_amount: Option<u64> = match result["amount"].as_u64(){
            Some(v)=>Some(v),
            None => None
        };
        let refund_id: Option<String> = match result["id"].as_str(){
            Some(v)=>Some(v.to_string()),
            None => None
        };
        let param = OederRefund{
            order_id, // 订单id号
            refund_amount, // 退款金额
            refund_id
        };
        return serde_json::to_value(param).unwrap();
    }else{
        return serde_json::from_str("error").unwrap();
    }
}
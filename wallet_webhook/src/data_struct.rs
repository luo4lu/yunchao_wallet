use crate::config;
use mysql_async::{Row, Pool};
use mysql_async::prelude::Queryable;
use serde::{Serialize};
use chrono::{NaiveDateTime};
use log::{warn};
//充值对象结构
#[derive(Debug, Serialize)]
pub struct RechargeServer{
    pub id: String,
    #[serde(rename = "type")]
    pub recharge_type: String,
    pub created: i64,
    pub amount: i64,
    pub recharge_amount: i64,
    pub fee: i64,
    pub succeeded: bool,
    pub time_succeeded: Option<i64>,
    pub wallet_id: String,
    pub description: Option<String>,
    pub extra: Option<serde_json::Value>,
    pub settle: String
}
impl RechargeServer {
    pub async fn new(object_id: String) -> Result<Self, &'static str> {
        //数据库查询语句
        let sql_str = format!("select id, type, created, amount, recharge_amount, fee, secceeded, 
        time_succeeded, wallet_id, description, extra, settle from recharge where id = \'{}\'",object_id);
         //数据库连接
        let pool: Pool = config::get_db();
        let mut conn = pool.get_conn().await.unwrap();
        let row: Vec<Row> = conn.query(sql_str).await.unwrap();
        if row.is_empty(){
            warn!("recharge select failed！！");
            return Err("recharge select failed！！");
        }
        //释放资源
        drop(conn);
        pool.disconnect().await.unwrap();

        let create_time: NaiveDateTime = row[0].get(2).unwrap();
        let time_s: Option<NaiveDateTime> = row[0].get(7);
        let time_succeeded: Option<i64>;
        if time_s.is_some(){
            time_succeeded = Some(time_s.unwrap().timestamp());
        }else{
            time_succeeded = None;
        }
        Ok(Self{
            id: row[0].get(0).unwrap(),
            recharge_type: row[0].get(1).unwrap(),
            created: create_time.timestamp(),
            amount: row[0].get(3).unwrap(),
            recharge_amount: row[0].get(4).unwrap(),
            fee: row[0].get(5).unwrap(),
            succeeded: row[0].get(6).unwrap(),
            time_succeeded,
            wallet_id: row[0].get(8).unwrap(),
            description: row[0].get(9),
            extra: row[0].get(10),
            settle: row[0].get(11).unwrap()
        })
    }
}
//钱包提现数据结构
#[derive(Debug, Serialize)]
pub struct WithdrawServer{
    pub id: String,
    #[serde(rename = "type")]
    pub wallet_type: String,
    pub created: i64,
    pub amount: i64,
    pub description: Option<String>,
    pub status: String,
    pub wallet_id: String,
    pub settle: String,
    pub time_canceled: Option<i64>,
    pub time_succeeded: Option<i64>,
    pub extra: Option<serde_json::Value>
}
impl WithdrawServer {
    pub async fn new(object_id: String) -> Result<Self, &'static str> {
        //数据库查询语句
        let sql_str = format!("select id, type, created, amount, description, status, wallet_id, 
        settle, time_canceled, time_succeeded, extra from withdraw where id = \'{}\'",object_id);
         //数据库连接
        let pool: Pool = config::get_db();
        let mut conn = pool.get_conn().await.unwrap();
        let row: Vec<Row> = conn.query(sql_str).await.unwrap();
        if row.is_empty(){
            warn!("withdraw select failed！！");
            return Err("withdraw select failed！！");
        }
        //释放资源
        drop(conn);
        pool.disconnect().await.unwrap();

        let create_time: NaiveDateTime = row[0].get(2).unwrap();
        let time_c: Option<NaiveDateTime> = row[0].get(8);
        let time_canceled: Option<i64>;
        if time_c.is_some(){
            time_canceled = Some(time_c.unwrap().timestamp());
        }else{
            time_canceled = None;
        }
        let time_s: Option<NaiveDateTime> = row[0].get(9);
        let time_succeeded: Option<i64>;
        if time_s.is_some(){
            time_succeeded = Some(time_s.unwrap().timestamp());
        }else{
            time_succeeded = None;
        }
        Ok(Self{
            id: row[0].get(0).unwrap(),
            wallet_type: row[0].get(1).unwrap(),
            created: create_time.timestamp(),
            amount: row[0].get(3).unwrap(),
            description: row[0].get(4),
            status: row[0].get(5).unwrap(),
            wallet_id: row[0].get(6).unwrap(),
            settle: row[0].get(7).unwrap(),
            time_canceled,
            time_succeeded,
            extra: row[0].get(10),
        })
    }
}
//转账服务数据结构
#[derive(Debug, Serialize)]
pub struct TransferServer{
    pub id: String,
    #[serde(rename = "type")]
    pub transfer_type: String,
    pub created: i64,
    pub amount: i64,
    pub fee: i64,
    pub fee_wallet: String,
    pub wallet_id: String,
    pub to_wallet: String,
    pub description: Option<String>,
    pub status: String,
    pub extra: Option<serde_json::Value>
}
impl TransferServer {
    pub async fn new(object_id: String) -> Result<Self, &'static str> {
        //数据库查询语句
        let sql_str = format!("select id, type, created, amount, fee, fee_wallet, wallet_id, to_wallet, 
        description, status, extra from transfer where id = \'{}\'",object_id);
         //数据库连接
        let pool: Pool = config::get_db();
        let mut conn = pool.get_conn().await.unwrap();
        let row: Vec<Row> = conn.query(sql_str).await.unwrap();
        if row.is_empty(){
            warn!("transfer select failed！！");
            return Err("transfer select failed！！");
        }
        //释放资源
        drop(conn);
        pool.disconnect().await.unwrap();

        let create_time: NaiveDateTime = row[0].get(2).unwrap();
        
        Ok(Self{
            id: row[0].get(0).unwrap(),
            transfer_type: row[0].get(1).unwrap(),
            created: create_time.timestamp(),
            amount: row[0].get(3).unwrap(),
            fee: row[0].get(4).unwrap(),
            fee_wallet: row[0].get(5).unwrap(),
            wallet_id: row[0].get(6).unwrap(),
            to_wallet: row[0].get(7).unwrap(),
            description: row[0].get(8),
            status: row[0].get(9).unwrap(),
            extra: row[0].get(10),
        })
    }
}
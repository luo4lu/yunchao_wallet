use crate::response::ResponseBody;
use actix_web::{post, web, HttpResponse, Responder};
use chrono::prelude::*;
use chrono::{NaiveDateTime,DateTime};
use deadpool_postgres::Pool;
use log::{warn,info};
use serde::{Deserialize, Serialize};
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

/*
 *function: 创建转账对象
 * param：
 * data: 数据库连接句柄
 * req：数据请求结构
 *
 * return :响应数据code=0成功，其他值参考错误列表
 */
#[derive(Deserialize, Debug)]
pub struct TransferRequest{
    extra: Option<serde_json::Value>,
    wallet_id: String,
    to_wallet: String,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransferResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub ttype: String,
    pub created: NaiveDateTime,
    pub extra: Option<serde_json::Value>,
    pub to_wallet: String,
    pub wallet_id: String,
    pub description: Option<String>,
    pub status: String
}
#[post("/wallets/transfers")]
pub async fn create_payment(
    data: web::Data<Pool>,
    req: web::Json<TransferRequest>
) -> impl Responder {
    //获取数据库句柄
    let conn = data.get().await.unwrap();
    //生成支付对象id
    let local: DateTime<Local> = Local::now();
    let context = Context::new(42);
    let ts = Timestamp::from_unix(&context, local.second() as u64, local.nanosecond());
    let trans_uuid = Uuid::new_v1(
        ts,
        &[
            't' as u8, 'r' as u8, 'a' as u8, 'n' as u8, 's' as u8, 'f' as u8,
        ],
    )
    .expect("failed to generate order UUID");
    let trans_uuid = trans_uuid.to_string();
    let trans_type = String::from("transfer");
    let status = String::from("created");
    match conn.query("INSERT INTO transfer(id, type, created, wallet_id, to_wallet, status, update_time) 
    VALUES($1, $2, now(), $3, $4, $5, now())",
    &[&trans_uuid, &trans_type, &req.wallet_id, &req.wallet_id, &status]).await{
        Ok(_) => {
            info!("create transfer object success!!!");
        }
        Err(error) => {
            warn!("1.insert transfer data failed for transfer system of:{:?}",error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };
    if req.description.is_some(){
        let description_info: String = req.description.as_ref().unwrap().to_string();
        match conn
        .query(
            "UPDATE transfer SET description = $1 where id = $2",
            &[&description_info, &trans_uuid]
        ).await{
            Ok(_) => {},
            Err(error) => {
                warn!("1.update transfer data failed for description of:{:?}",error);
                return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
            }
        };
    }
    if req.extra.is_some(){
        let extra_info: serde_json::Value = serde_json::to_value(req.extra.as_ref().unwrap()).unwrap();
        match conn
        .query(
            "UPDATE transfer SET extra = $1 where id = $2",
            &[&extra_info, &trans_uuid]
        ).await{
            Ok(_) => {},
            Err(error) => {
                warn!("1.update transfer data failed for extra of:{:?}",error);
                return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
            }
        };
    }
    let trans_info = match conn.query("SELECT created from transfer where id = $1",&[&trans_uuid]).await{
        Ok(value) =>{
            value
        }
        Err(error)=>{
            warn!("1.get payment select info id failed:{:?}",error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };
    if trans_info.is_empty() {
        warn!("select transfer is empty from id");
        return HttpResponse::Ok().json(ResponseBody::<()>::object_not_exit());
    }
    return HttpResponse::Ok().json(ResponseBody::<TransferResponse>::new_success(Some(TransferResponse{
        id: trans_uuid,
        ttype: trans_type,
        created: trans_info[0].get(0),
        extra: req.extra.clone(),
        to_wallet: req.to_wallet.clone(),
        wallet_id: req.wallet_id.clone(),
        description: req.description.clone(),
        status
    })));
}
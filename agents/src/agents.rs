use crate::response::ResponseBody;
use actix_web::{post, delete, web, HttpResponse, Responder, HttpRequest};
use chrono::prelude::*;
use chrono::{NaiveDateTime,DateTime};
use deadpool_postgres::Pool;
use log::{warn,info};
use serde::{Deserialize, Serialize};
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

/*
 *function: 创建签约对象
 * param：
 * data: 数据库连接句柄
 * req：数据请求结构
 *
 * return :响应数据code=0成功，其他值参考错误列表
 */

#[derive(Deserialize, Debug)]
pub struct AgentsRequest{
    extra: Option<serde_json::Value>,
    to_wallet: String,
    description: Option<String>,
    begin_time: i64,
    end_time: i64,
    limit_amount: i64,
    day_limit_amount: Option<i64>,
    month_limit_amount: Option<i64>,
    total_limit_amount: Option<i64>
}
#[derive(Debug, Serialize)]
pub struct AgentsResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub ttype: String,
    pub created: i64,
    pub extra: Option<serde_json::Value>,
    pub from_wallet: String,
    pub to_wallet: String,
    pub begin_time: i64,
    pub end_time: i64,
    pub limit_amount: i64,
    pub day_limit_amount: Option<i64>,
    pub month_limit_amount: Option<i64>,
    pub total_limit_amount: Option<i64>,
    pub description: Option<String>,
}

#[post("/wallets/{wallet_id}/agents")]
pub async fn create_agents(
    data: web::Data<Pool>,
    req: web::Json<AgentsRequest>,
    req_head: HttpRequest
) -> impl Responder {
    let op1 = req_head.match_info().get("wallet_id");

    if op1.is_none() {
        return HttpResponse::Ok().json(ResponseBody::<()>::return_none_error());
    }

    let wallet_id = op1.unwrap();
    //获取数据库句柄
    let conn = data.get().await.unwrap();
    //生成支付对象id
    let local: DateTime<Local> = Local::now();
    let context = Context::new(42);
    let ts = Timestamp::from_unix(&context, local.second() as u64, local.nanosecond());
    let agents_uuid = Uuid::new_v1(
        ts,
        &[
            'a' as u8, 'g' as u8, 'e' as u8, 'n' as u8, 't' as u8, 's' as u8,
        ],
    )
    .expect("failed to generate order UUID");
    let agents_uuid = agents_uuid.to_string();
    let agents_type = String::from("agent");
    let dt_begin = NaiveDateTime::from_timestamp(req.begin_time,0);
    let dt_end = NaiveDateTime::from_timestamp(req.end_time,0);
    match conn.query("INSERT INTO agents(id, type, created, from_wallet, to_wallet, begin_time, end_time, limit_amount, update_time) 
    VALUES($1, $2, now(), $3, $4, $5, $6, $7, now())", &[&agents_uuid, &agents_type, &wallet_id, &req.to_wallet, &dt_begin, &dt_end, &req.limit_amount]).await{
        Ok(_) => {
            info!("create agents object success!!!");
        }
        Err(error) => {
            warn!("1.insert agents data failed for agents system of:{:?}",error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };
    if req.description.is_some(){
        let description_info: String = req.description.as_ref().unwrap().to_string();
        match conn.query("UPDATE agents SET description = $1 where id = $2",
            &[&description_info, &agents_uuid]
        ).await{
            Ok(_) => {},
            Err(error) => {
                warn!("1.update agents data failed for description of:{:?}",error);
                return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
            }
        };
    }
    if req.extra.is_some(){
        let extra_info: serde_json::Value = serde_json::to_value(req.extra.as_ref().unwrap()).unwrap();
        match conn
        .query(
            "UPDATE agents SET extra = $1 where id = $2",
            &[&extra_info, &agents_uuid]
        ).await{
            Ok(_) => {},
            Err(error) => {
                warn!("2.update agents data failed for extra of:{:?}",error);
                return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
            }
        };
    }
    if req.day_limit_amount.is_some(){
        let day: i64 = req.day_limit_amount.unwrap();
        match conn
        .query(
            "UPDATE agents SET day_limit_amount = $1 where id = $2",
            &[&day, &agents_uuid]
        ).await{
            Ok(_) => {},
            Err(error) => {
                warn!("3.update agents data failed for day of:{:?}",error);
                return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
            }
        };
    }
    if req.month_limit_amount.is_some(){
        let month: i64 = req.month_limit_amount.unwrap();
        match conn
        .query(
            "UPDATE agents SET month_limit_amount = $1 where id = $2",
            &[&month, &agents_uuid]
        ).await{
            Ok(_) => {},
            Err(error) => {
                warn!("4.update agents data failed for month_limit_amount of:{:?}",error);
                return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
            }
        };
    }
    if req.total_limit_amount.is_some(){
        let total: i64 = req.total_limit_amount.unwrap();
        match conn
        .query(
            "UPDATE agents SET total_limit_amount = $1 where id = $2",
            &[&total, &agents_uuid]
        ).await{
            Ok(_) => {},
            Err(error) => {
                warn!("5.update agents data failed for total_limit_amount of:{:?}",error);
                return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
            }
        };
    }
    let agents_info = match conn.query("SELECT created from agents where id = $1",&[&agents_uuid]).await{
        Ok(value) =>{
            value
        }
        Err(error)=>{
            warn!("1.get agents select info id failed:{:?}",error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };
    let created_time: NaiveDateTime = agents_info[0].get(0);
    return HttpResponse::Ok().json(ResponseBody::<AgentsResponse>::new_success(Some(AgentsResponse{
        id: agents_uuid,
        ttype: agents_type,
        created: created_time.timestamp(),
        extra: req.extra.clone(),
        from_wallet: wallet_id.to_string(),
        to_wallet: req.to_wallet.clone(),
        begin_time: req.begin_time,
        end_time: req.end_time,
        limit_amount: req.limit_amount,
        day_limit_amount: req.day_limit_amount,
        month_limit_amount: req.month_limit_amount,
        total_limit_amount: req.total_limit_amount,
        description: req.description.clone(),
    })));
}

/*
 *function: 撤销支付对象
 * param：
 * data: 数据库连接句柄
 * req：数据请求结构
 *
 * return :响应数据code=0成功，其他值参考错误列表
 */

#[delete("/wallets/{wallet_id}/agents/{id}")]
pub async fn delete_agents_id(
    data: web::Data<Pool>,
    req_head: HttpRequest
) ->impl Responder {
    let op1 = req_head.match_info().get("wallet_id");
    let op2 = req_head.match_info().get("id");
    if op1.is_none() || op2.is_none(){
        return HttpResponse::Ok().json(ResponseBody::<()>::return_none_error());
    }

    let wallet_id = op1.unwrap();
    let id = op2.unwrap();
    //获取数据库句柄
    let conn = data.get().await.unwrap();
    let agents_select = match conn.query("SELECT type, created, extra, to_wallet, begin_time, end_time, limit_amount, day_limit_amount, 
    month_limit_amount, total_limit_amount,description from agents where id = $1 and from_wallet = $2",&[&id, &wallet_id]).await{
        Ok(value) =>{
            value
        }
        Err(error)=>{
            warn!("1.get agents select info id failed:{:?}",error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };
    if agents_select.is_empty() {
        warn!("select agents_select is empty from id");
        return HttpResponse::Ok().json(ResponseBody::<()>::object_not_exit());
    }
    match conn.query("DELETE FROM agents WHERE id = $1 and from_wallet = $2",&[&id, &wallet_id]).await{
        Ok(_) =>{
            let created_time: NaiveDateTime = agents_select[0].get(1);
            let dt_begin: NaiveDateTime = agents_select[0].get(4);
            let dt_end: NaiveDateTime = agents_select[0].get(5);
            return HttpResponse::Ok().json(ResponseBody::<AgentsResponse>::new_success(Some(AgentsResponse{
                id: id.to_string(),
                ttype: agents_select[0].get(0),
                created: created_time.timestamp(),
                extra: agents_select[0].get(2),
                from_wallet: wallet_id.to_string(),
                to_wallet: agents_select[0].get(3),
                begin_time: dt_begin.timestamp(),
                end_time: dt_end.timestamp(),
                limit_amount: agents_select[0].get(6),
                day_limit_amount: agents_select[0].get(7),
                month_limit_amount: agents_select[0].get(8),
                total_limit_amount: agents_select[0].get(9),
                description: agents_select[0].get(10),
            })));
        }
        Err(error)=>{
            warn!("delete agents wallet id for id failed:{:?}",error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };  
}
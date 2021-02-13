use crate::transfer::TransferResponse;
use crate::response::ResponseBody;
use actix_web::{get, web, HttpResponse, Responder};
use chrono::{NaiveDateTime};
use deadpool_postgres::Pool;
use log::{warn};
use serde::{Deserialize, Serialize};

/*
 *function: 查询交易对象详情
 *param:
 *data: 连接数据库句柄
 *
 *return：响应数据code=0成功，其他值参照错误列表
*/

#[derive(Debug, Deserialize)]
pub struct GetTransObject {
    pub trans_id: String,
}
#[get("/wallets/transfers/info")]
pub async fn get_trans_info(
    data: web::Data<Pool>,
    req: web::Query<GetTransObject>
) ->impl Responder {
    //获取数据库句柄
    let conn = data.get().await.unwrap();
    let trans_select = match conn.query("SELECT id, type, created, extra, wallet_id, to_wallet, description, 
    status from transfer where id = $1",&[&req.trans_id]).await{
        Ok(value) =>{
            value
        }
        Err(error)=>{
            warn!("1.get payment select info id failed:{:?}",error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };
    if trans_select.is_empty() {
        warn!("select trans_select is empty from id");
        return HttpResponse::Ok().json(ResponseBody::<()>::object_not_exit());
    }
    return HttpResponse::Ok().json(ResponseBody::<TransferResponse>::new_success(Some(TransferResponse{
        id: trans_select[0].get(0),
        ttype: trans_select[0].get(1),
        created: trans_select[0].get(2),
        extra: trans_select[0].get(3),
        to_wallet: trans_select[0].get(5),
        wallet_id: trans_select[0].get(4),
        description: trans_select[0].get(6),
        status: trans_select[0].get(7)
    })));
}

/*
 *function: 查询交易对象列表
 *param:
 *data: 连接数据库句柄
 *
 *return：响应数据code=0成功，其他值参照错误列表
*/
#[derive(Debug, Deserialize)]
pub struct GetTransObjectQuery {
    page: i64,
    count: i64,
    begin_time: Option<NaiveDateTime>,
    end_time: Option<NaiveDateTime>,
}
//查询结果响应数据
#[derive(Debug, Serialize)]
pub struct ObjectIdResult {
    total: i64,
    data: Vec<TransferResponse>,
}

#[get("/wallets/transfers/list")]
pub async fn get_trans_list(
    data: web::Data<Pool>,
    req: web::Query<GetTransObjectQuery>
) ->impl Responder {
    //查询页数计算
    let page_num: i64 = (req.page - 1) * req.count;
    //获取数据库句柄
    let conn = data.get().await.unwrap();
    let mut sql_sum = "SELECT count(*) from transfer".to_string();
    let mut sql = "SELECT id, type, created, extra, wallet_id, to_wallet, description, 
    status from transfer".to_string();
    let mut sql_params: Vec<&(dyn tokio_postgres::types::ToSql + std::marker::Sync)> = vec![];
    if req.begin_time.is_some() && req.end_time.is_some() {
        sql_sum.push_str(" where created > $");
        sql_sum.push_str(&(sql_params.len() + 1).to_string());
        sql.push_str(" where created > $");
        sql.push_str(&(sql_params.len() + 1).to_string());
        sql_params.push(req.begin_time.as_ref().unwrap());
        sql_sum.push_str(" and created < $");
        sql_sum.push_str(&(sql_params.len() + 1).to_string());
        sql.push_str(" and created < $");
        sql.push_str(&(sql_params.len() + 1).to_string());
        sql_params.push(req.end_time.as_ref().unwrap());
    }else if req.end_time.is_some() {
        sql_sum.push_str(" where created < $");
        sql_sum.push_str(&(sql_params.len() + 1).to_string());
        sql.push_str(" where created < $");
        sql.push_str(&(sql_params.len() + 1).to_string());
        sql_params.push(req.end_time.as_ref().unwrap());
    }else if req.begin_time.is_some() {
        sql_sum.push_str(" where created > $");
        sql_sum.push_str(&(sql_params.len() + 1).to_string());
        sql.push_str(" where created > $");
        sql.push_str(&(sql_params.len() + 1).to_string());
        sql_params.push(req.begin_time.as_ref().unwrap());
    }
    //查询条件范围订单总条数
    let select_total = match conn.query(sql_sum.as_str(), &sql_params[..]).await {
        Ok(value) => value,
        Err(error) => {
            warn!("1.get payment select data failed:{:?}", error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(
                error.to_string(),
            ));
        }
    };
    let total_pay: i64 = select_total[0].get(0);
    if total_pay <= 0 {
        warn!("The user has not recharged any payment object");
        return HttpResponse::Ok().json(ResponseBody::<i32>::new_success(Some(0)));
    }
    sql.push_str(" ORDER BY created DESC LIMIT $");
    sql.push_str(&(sql_params.len()+1).to_string());
    sql.push_str(" OFFSET $");
    sql.push_str(&(sql_params.len() + 2).to_string());
    sql_params.push(&req.count);
    sql_params.push(&page_num);
    //查询数据
    let trans_select = match conn.query(sql.as_str(), &sql_params[..]).await {
        Ok(value) => value,
        Err(error) => {
            warn!("2.get payment info select data failed:{:?}", error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(
                error.to_string(),
            ));
        }
    };
    let mut trans_data: Vec<TransferResponse> = Vec::new();
    for value in trans_select.iter(){
        let params: TransferResponse = TransferResponse {
            id: value.get(0),
            ttype: value.get(1),
            created: value.get(2),
            extra: value.get(3),
            to_wallet: value.get(5),
            wallet_id: value.get(4),
            description: value.get(6),
            status: value.get(7)
        };
        trans_data.push(params);
    }
    HttpResponse::Ok().json(ResponseBody::<ObjectIdResult>::new_success(Some(
        ObjectIdResult {
            total: total_pay,
            data: trans_data,
        },
    )))
}
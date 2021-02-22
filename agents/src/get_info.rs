use crate::agents::AgentsResponse;
use crate::response::ResponseBody;
use actix_web::{get, web, HttpResponse, Responder, HttpRequest};
use chrono::{NaiveDateTime};
use deadpool_postgres::Pool;
use log::{warn};
use serde::{Deserialize, Serialize};

/*
 *function: 查询签约对象详情
 *param:
 *data: 连接数据库句柄
 *
 *return：响应数据code=0成功，其他值参照错误列表
*/
#[get("/wallets/{wallet_id}/agents/{id}")]
pub async fn get_agents_info(
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
    return HttpResponse::Ok().json(ResponseBody::<AgentsResponse>::new_success(Some(AgentsResponse{
        id: id.to_string(),
        ttype: agents_select[0].get(0),
        created: agents_select[0].get(1),
        extra: agents_select[0].get(2),
        from_wallet: wallet_id.to_string(),
        to_wallet: agents_select[0].get(3),
        begin_time: agents_select[0].get(4),
        end_time: agents_select[0].get(5),
        limit_amount: agents_select[0].get(6),
        day_limit_amount: agents_select[0].get(7),
        month_limit_amount: agents_select[0].get(8),
        total_limit_amount: agents_select[0].get(9),
        description: agents_select[0].get(10),
    })));
}

/*
 *function: 查询签约对象列表
 *param:
 *data: 连接数据库句柄
 *
 *return：响应数据code=0成功，其他值参照错误列表
*/
#[derive(Debug, Deserialize)]
pub struct GetAgentsObjectQuery {
    page: i64,
    count: i64,
    begin_time: Option<NaiveDateTime>,
    end_time: Option<NaiveDateTime>,
}
//查询结果响应数据
#[derive(Debug, Serialize)]
pub struct ObjectIdResult {
    total: i64,
    data: Vec<AgentsResponse>,
}

#[get("/wallets/{wallet_id}/agents")]
pub async fn get_agents_list(
    data: web::Data<Pool>,
    req: web::Query<GetAgentsObjectQuery>,
    req_head: HttpRequest
) ->impl Responder {
    let op1 = req_head.match_info().get("wallet_id");

    if op1.is_none() {
        return HttpResponse::Ok().json(ResponseBody::<()>::return_none_error());
    }

    let wallet_id = op1.unwrap();
    //查询页数计算
    let page_num: i64 = (req.page - 1) * req.count;
    //获取数据库句柄
    let conn = data.get().await.unwrap();
    let mut sql_sum = "SELECT count(*) from agents where from_wallet = $1".to_string();
    let mut sql = "SELECT id, type, created, extra, from_wallet, to_wallet, begin_time, end_time, limit_amount, day_limit_amount, 
    month_limit_amount, total_limit_amount,description from agents where from_wallet = $1".to_string();
    let mut sql_params: Vec<&(dyn tokio_postgres::types::ToSql + std::marker::Sync)> = vec![&wallet_id];
    if req.begin_time.is_some() && req.end_time.is_some() {
        sql_sum.push_str(" and created > $");
        sql_sum.push_str(&(sql_params.len() + 1).to_string());
        sql.push_str(" and created > $");
        sql.push_str(&(sql_params.len() + 1).to_string());
        sql_params.push(req.begin_time.as_ref().unwrap());
        sql_sum.push_str(" and created < $");
        sql_sum.push_str(&(sql_params.len() + 1).to_string());
        sql.push_str(" and created < $");
        sql.push_str(&(sql_params.len() + 1).to_string());
        sql_params.push(req.end_time.as_ref().unwrap());
    }else if req.end_time.is_some() {
        sql_sum.push_str(" and created < $");
        sql_sum.push_str(&(sql_params.len() + 1).to_string());
        sql.push_str(" and created < $");
        sql.push_str(&(sql_params.len() + 1).to_string());
        sql_params.push(req.end_time.as_ref().unwrap());
    }else if req.begin_time.is_some() {
        sql_sum.push_str(" and created > $");
        sql_sum.push_str(&(sql_params.len() + 1).to_string());
        sql.push_str(" and created > $");
        sql.push_str(&(sql_params.len() + 1).to_string());
        sql_params.push(req.begin_time.as_ref().unwrap());
    }
    //查询条件范围订单总条数
    let select_total = match conn.query(sql_sum.as_str(), &sql_params[..]).await {
        Ok(value) => value,
        Err(error) => {
            warn!("1.get agents select data failed:{:?}", error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(
                error.to_string(),
            ));
        }
    };
    let total_pay: i64 = select_total[0].get(0);
    if total_pay <= 0 {
        warn!("The user has not recharged any agents object");
        return HttpResponse::Ok().json(ResponseBody::<i32>::new_success(Some(0)));
    }
    sql.push_str(" ORDER BY created DESC LIMIT $");
    sql.push_str(&(sql_params.len()+1).to_string());
    sql.push_str(" OFFSET $");
    sql.push_str(&(sql_params.len() + 2).to_string());
    sql_params.push(&req.count);
    sql_params.push(&page_num);
    //查询数据
    let agents_select = match conn.query(sql.as_str(), &sql_params[..]).await {
        Ok(value) => value,
        Err(error) => {
            warn!("2.get payment info select data failed:{:?}", error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(
                error.to_string(),
            ));
        }
    };
    let mut agents_data: Vec<AgentsResponse> = Vec::new();
    for value in agents_select.iter(){
        let params: AgentsResponse = AgentsResponse {
            id: value.get(0),
            ttype: value.get(1),
            created: value.get(2),
            extra: value.get(3),
            from_wallet: value.get(4),
            to_wallet: value.get(5),
            begin_time: value.get(6),
            end_time: value.get(7),
            limit_amount: value.get(8),
            day_limit_amount: value.get(9),
            month_limit_amount: value.get(10),
            total_limit_amount: value.get(11),
            description: value.get(12),
        };
        agents_data.push(params);
    }
    HttpResponse::Ok().json(ResponseBody::<ObjectIdResult>::new_success(Some(
        ObjectIdResult {
            total: total_pay,
            data: agents_data,
        },
    )))
}
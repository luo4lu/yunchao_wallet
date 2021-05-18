use crate::config;
use crate::response::ResponseBody;
use actix_web::{get,delete, HttpResponse, Responder, HttpRequest};
use chrono::{NaiveDateTime};
use log::{warn};
use serde::{Serialize};
use mysql_async::{ Row, Pool};
use mysql_async::prelude::Queryable;

/*
 *function: 查询指定的wallet id是否存在url或者失败原因
 *param:
 *
 *
 *return：响应数据code=0成功，其他值参照错误列表
*/

#[derive(Serialize, Debug)]
pub struct SuccessResponse{
    status: String, //filed，success
    wallet_id: String,
    reseaon: Option<String>,
    url: Option<String>,
    create: i64
}

#[get("/user/{wallet_id}/url")]
pub async fn get_url(
    req_head: HttpRequest
) -> impl Responder {

    let op1 = req_head.match_info().get("wallet_id");
    if op1.is_none(){
        return HttpResponse::Ok().json(ResponseBody::<()>::return_none_error());
    }
    let id = op1.unwrap();
    //数据库连接
    let pool: Pool = config::get_db();
    let mut conn = pool.get_conn().await.unwrap();
    let sql_str = format!("select status, reseaon, url, create_time from url_notify where wallet_id = \'{}\'",id);
    let row: Vec<Row> = match conn.query(sql_str).await{
        Ok(v) => v,
        Err(error)=>{
            warn!("select user_info check web url failed:{}",error.to_string());
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };
    if row.is_empty(){
        warn!("select failed！！");
        return HttpResponse::Ok().json(ResponseBody::<()>::object_not_exit())
    }
    let status: String = row[0].get(0).unwrap();
    let reseaon: Option<String> = row[0].get(1).unwrap();
    let url: Option<String> =row[0].get(2).unwrap();
    let create_time: NaiveDateTime = row[0].get(3).unwrap();
    //释放连接
    drop(conn);
    pool.disconnect().await.unwrap();
    return HttpResponse::Ok().json(ResponseBody::<SuccessResponse>::new_success(Some(SuccessResponse{
        status,
        wallet_id: id.to_owned(),
        reseaon,
        url,
        create: create_time.timestamp()
    })));
}


#[delete("/user/{wallet_id}/url")]
pub async fn delete_url(req_head: HttpRequest)->impl Responder {

    let op1 = req_head.match_info().get("wallet_id");
    if op1.is_none(){
        return HttpResponse::Ok().json(ResponseBody::<()>::return_none_error());
    }
    let id = op1.unwrap();
    //数据库连接
    let pool: Pool = config::get_db();
    let mut conn = pool.get_conn().await.unwrap();
    let del_str = format!("delete from url_notify where wallet_id = \'{}\'",id);
    let _:Vec<Row> = match conn.query(del_str).await{
        Ok(v) => {
            v
        },
        Err(error)=>{
            warn!("delete url_notify url status failed:{}",error.to_string());
            drop(conn);
            pool.disconnect().await.unwrap();
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };
    drop(conn);
    pool.disconnect().await.unwrap();
    return HttpResponse::Ok().json(ResponseBody::<()>::new_success(None));
}
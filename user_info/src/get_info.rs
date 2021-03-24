use crate::config;
use crate::response::ResponseBody;
use actix_web::{get, HttpResponse, Responder, HttpRequest};
use chrono::{NaiveDateTime};
use log::{warn};
use serde::{Deserialize, Serialize};
use mysql_async::{ Row, Pool};
use mysql_async::prelude::Queryable;

/*
 *function: 查询详情
 *param:
 *
 *
 *return：响应数据code=0成功，其他值参照错误列表
*/
#[derive(Serialize, Debug)]
 pub struct UserInfoResponse{
    appid: String, // 用户应用id
    web_url: String,
    create: i64
 }

#[derive(Debug, Deserialize)]
pub struct GetTransObject {
    pub trans_id: String,
}
#[get("/user/info/{appid}")]
pub async fn get_user_info(
    req_head: HttpRequest
) -> impl Responder {

    let op1 = req_head.match_info().get("appid");
    if op1.is_none(){
        return HttpResponse::Ok().json(ResponseBody::<()>::return_none_error());
    }
    let id = op1.unwrap();
    //数据库连接
    let pool: Pool = config::get_db();
    let mut conn = pool.get_conn().await.unwrap();
    let sql_str = format!("select appid, web_url, create_time from user_info where appid = \'{}\'",id);
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
    drop(conn);
    pool.disconnect().await.unwrap();
    let appid: String = row[0].get(0).unwrap();
    let web_url: String = row[0].get(1).unwrap();
    let created_time: NaiveDateTime = row[0].get(2).unwrap();
    return HttpResponse::Ok().json(ResponseBody::<UserInfoResponse>::new_success(Some(UserInfoResponse{
        appid, // 用户应用id
        web_url,
        create: created_time.timestamp()
     })));
}




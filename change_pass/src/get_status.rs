use crate::config;
use crate::response::ResponseBody;
use crate::url_status::InfoResponse;
use actix_web::{get, HttpResponse, Responder, HttpRequest};
use log::{warn};
use serde::{Deserialize};
use mysql_async::{ Row, Pool};
use mysql_async::prelude::Queryable;

#[derive(Debug, Deserialize)]
pub struct GetTransObject {
    pub user_id: String,
}
#[get("/change/status/{appid}")]
pub async fn get_info(
    req_head: HttpRequest
) -> impl Responder {
    let op1 = req_head.match_info().get("userid");
    if op1.is_none(){
        return HttpResponse::Ok().json(ResponseBody::<()>::return_none_error());
    }
    let id = op1.unwrap();
    //数据库连接
    let pool: Pool = config::get_db();
    let mut conn = pool.get_conn().await.unwrap();
    let sql_str = format!("select user_id, url, status from url_statu where user_id = \'{}\'",id);
    let row: Vec<Row> = match conn.query(sql_str).await{
        Ok(v) => v,
        Err(error)=>{
            warn!("select user_info check web url failed:{}",error.to_string());
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };
    if row.is_empty(){
        warn!("this user id is none!!");
        drop(conn);
        pool.disconnect().await.unwrap();   
        return HttpResponse::Ok().json(ResponseBody::<()>::object_not_exit())
    }
    let user_id: String = row[0].get(0).unwrap();
    let url: String = row[0].get(1).unwrap();
    let mut status: bool = row[0].get(2).unwrap();
    if status == false {
        let sql_update = format!("UPDATE url_statu SET status={}, update_time=now() where user_id = \'{}\'",true, id);
        let _:Vec<Row> = match conn.query(sql_update).await{
            Ok(v) => {
                info!("update url_statu status success!!");
                v
            },
            Err(error)=>{
                warn!("update url_statu check url status failed:{}",error.to_string());
                drop(conn);
                pool.disconnect().await.unwrap();
                return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
            }
        };
        status = true;
    }
    return HttpResponse::Ok().json(ResponseBody::<InfoResponse>::new_success(Some(InfoResponse{
        id: user_id, // 用户应用id
        url,
        status
     })));
}
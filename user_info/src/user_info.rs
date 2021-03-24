use crate::config;
use crate::response::ResponseBody;
use actix_web::{post, web, HttpResponse, Responder};
use log::{warn};
use serde::{Deserialize, Serialize};
use mysql_async::{ params, Pool};
use mysql_async::prelude::Queryable;

/*
 * function：接收用户信息保存
 * param:
 * req: 数据请求结构
 * return： 响应数据code=0成功，其他值参考错误列表
 * 
 */

 #[derive(Deserialize, Debug)]
 pub struct UserInfoRequest{
    appid: String, // 用户应用id
    web_url: String
 }

 #[derive(Deserialize, Serialize, Debug)]
 pub struct UserInfoResponse{
    pub appid: String, // 用户应用id
    pub web_url: String
 }
 #[derive(Debug, PartialEq, Eq, Clone)]
 struct Payment {
    appid: String,
    web_url: String,
 }
 #[post("/user/info")]
 pub async fn user_info_save(
     req: web::Json<UserInfoRequest>
 )-> impl Responder {
    warn!("RECV={:?}",req);
    //连接数据库
    let pool: Pool = config::get_db();
    let mut conn = pool.get_conn().await.unwrap();
    let payments = vec![Payment{appid:req.appid.clone(), web_url: req.web_url.clone()}];
    let params = payments.clone().into_iter().map(|payment| {
        params! {
            "appid" => payment.appid,
            "web_url" => payment.web_url,
        }
    });
    match conn.exec_batch(r"INSERT INTO user_info(appid, web_url, create_time, update_time)
     VALUES(:appid, :web_url, now(),now())",params).await{
        Ok(_) =>{},
        Err(error) => {
            warn!("1.insert user_info data failed for user_info of:{:?}",error);
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
     };
    //释放连接
    drop(conn);
    pool.disconnect().await.unwrap();
    return HttpResponse::Ok().json(ResponseBody::<UserInfoResponse>::new_success(Some(UserInfoResponse{
        appid: req.appid.clone(), // 用户应用id
        web_url: req.web_url.clone()
     })));
 }
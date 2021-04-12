use crate::config;
use crate::response::ResponseBody;
use actix_web::{post, web, HttpResponse, Responder};
use log::{warn};
use serde::{Deserialize, Serialize};
use mysql_async::{ params, Pool, Row};
use mysql_async::prelude::Queryable;


/*
 * function：接收用户信息保存
 * param:
 * req: 数据请求结构
 * return： 响应数据code=0成功，其他值参考错误列表
 * 
 */
#[derive(Deserialize, Debug)]
 pub struct InfoRequest{
    id: String, // 用户id
    url: String
 }

 #[derive(Deserialize, Serialize, Debug)]
 pub struct InfoResponse{
    pub id: String, // 用户id
    pub url: String, 
    pub status: bool
 }
 #[derive(Debug, PartialEq, Eq, Clone)]
 struct SqlLoad {
    id: String,
    url: String,
    status: bool
 }
 #[post("/change/passworld")]
 pub async fn url_statu_save(
     req: web::Json<InfoRequest>
 ) ->impl Responder {
      //连接数据库
    let pool: Pool = config::get_db();
    let mut conn = pool.get_conn().await.unwrap();
    let sql_slc = format!("select status from url_statu where user_id = \'{}\'",req.id);
    let row: Vec<Row> = match conn.query(sql_slc).await{
      Ok(v) => v,
      Err(error)=>{
          warn!("select url_statu check url status failed:{}",error.to_string());
          drop(conn);
          pool.disconnect().await.unwrap();
          return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
      }
   };
   let status: bool = false;
   if row.is_empty(){
      //第一次修改
      let sqlload = vec![SqlLoad{id:req.id.clone(),url: req.url.clone(),status: status.clone()}];
      let params = sqlload.clone().into_iter().map(|sqlload| {
         params! {
             "user_id" => sqlload.id,
             "url" => sqlload.url,
             "status" => sqlload.status
         }
     });
     match conn.exec_batch(r"INSERT INTO url_statu(user_id, url, status, create_time, update_time)
     VALUES(:user_id, :url, :status, now(),now())",params).await{
        Ok(_) =>{},
        Err(error) => {
            warn!("1.insert url_statu data failed for url_statu of:{:?}",error);
            //释放连接
            drop(conn);
            pool.disconnect().await.unwrap();
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
     };
   }else{
      //第n次请求修改密码，n>1
      let sql_update = format!("UPDATE url_statu SET status={}, url=\'{}\', update_time=now() where user_id = \'{}\'",status, req.url, req.id);
      let _:Vec<Row> = match conn.query(sql_update).await{
         Ok(v) => {
            info!("update url_statu status success!!");
            v
         },
         Err(error)=>{
             warn!("select url_statu check url status failed:{}",error.to_string());
             drop(conn);
             pool.disconnect().await.unwrap();
             return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
         }
      };
   }
   //释放连接
   drop(conn);
   pool.disconnect().await.unwrap();
   return HttpResponse::Ok().json(ResponseBody::<InfoResponse>::new_success(Some(InfoResponse{
      id: req.id.clone(), // 用户应用id
      url: req.url.clone(),
      status
   })));
 }
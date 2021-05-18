use crate::config;
use crate::response::ResponseBody;
use actix_web::{post, web, HttpResponse, Responder};
use log::{warn};
use serde::{Deserialize, Serialize};
use mysql_async::{ Row, params, Pool};
use mysql_async::prelude::Queryable;

/*
 *function: 接收url数据
 *param: 
 * status: 验证成功或失败（filed，success）
 * wallet_id： 钱包id用来标识身份
 * reseaon：验证失败时传入的失败原因
 * url:验证成功时传入的用户通知url
 */
 
#[derive(Deserialize, Serialize, Debug)]
pub struct BnodeRequest{
    status: String, //filed，success
    wallet_id: String,
    reseaon: Option<String>, //filed时写明原因
    url: Option<String>
}
#[derive(Serialize, Debug)]
pub struct SuccessResponse{
    status: String, //filed，success
    wallet_id: String,
    reseaon: Option<String>,
    url: Option<String>
}
#[derive(Debug, PartialEq, Eq, Clone)]
 struct Payment {
    wallet_id: String,
    status: String,
    reseaon: Option<String>,
    url: Option<String>
 }
#[post("/user/url/notify")]
pub async fn user_notify(
    req: web::Json<BnodeRequest>
) -> impl Responder {
    //连接数据库
    let pool: Pool = config::get_db();
    let mut conn = pool.get_conn().await.unwrap();
    let reseaon: Option<String> = req.reseaon.clone();
    let url: Option<String> = req.url.clone();
    let payments = vec![Payment{wallet_id: req.wallet_id.clone(),status: req.status.clone(),reseaon:reseaon.clone(), url:url.clone()}];
    let params = payments.clone().into_iter().map(|payment| {
        params! {
            "wallet_id" => payment.wallet_id,
            "status" => payment.status,
            "reseaon" => payment.reseaon,
            "url" => payment.url
        }
    });
    let status: String = req.status.clone();
    if status == String::from("success"){
        if !req.url.is_some(){
            warn!("url is none,please input url");
            return HttpResponse::Ok().json(ResponseBody::<()>::url_is_none());
        }
    }else if status == String::from("failed"){
        if !req.reseaon.is_some() {
            warn!("failed reseaon is none,please input reseaon");
            return HttpResponse::Ok().json(ResponseBody::<()>::reseaon_is_none());
        }
    }else {
        warn!("Validation status error,input \"success\" or \"failed\"");
        return HttpResponse::Ok().json(ResponseBody::<()>::status_type_error());
    }
    let sql_str = format!("select url, create_time from url_notify where wallet_id = \'{}\'",req.wallet_id);
    let row: Vec<Row> = match conn.query(sql_str).await{
        Ok(v) => v,
        Err(error)=>{
            warn!("select user_info check web url failed:{}",error.to_string());
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
        }
    };
    if row.is_empty(){
        match conn.exec_batch(r"INSERT INTO url_notify(wallet_id, status, reseaon, url, create_time)
        VALUES(:wallet_id, :status, :reseaon, :url, now())",params).await{
            Ok(_) =>{},
            Err(error) => {
                warn!("1.insert url_notify data failed for url_notify of:{:?}",error);
                return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
            }
        };
    }else{
        let mut update_sql = String::new();
        if reseaon.is_some(){
            let tmp_reseaon = reseaon.clone().unwrap();
            update_sql = format!("update url_notify SET status=\'{}\',reseaon=\'{}\',create_time=now() where wallet_id = \'{}\'",
            status, tmp_reseaon, req.wallet_id);
        }
        if url.is_some(){
            let tmp_url = url.clone().unwrap();
            update_sql = format!("update url_notify SET status=\'{}\',url=\'{}\',create_time=now() where wallet_id = \'{}\'",
            status, tmp_url, req.wallet_id);
        }
        let _:Vec<Row> = match conn.query(update_sql).await{
            Ok(v) => {
                v
            },
            Err(error)=>{
                warn!("update url_notify check url status failed:{}",error.to_string());
                drop(conn);
                pool.disconnect().await.unwrap();
                return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
            }
        };
    }
    //释放连接
    drop(conn);
    pool.disconnect().await.unwrap();
    return HttpResponse::Ok().json(ResponseBody::<SuccessResponse>::new_success(Some(SuccessResponse{
        status,
        wallet_id: req.wallet_id.clone(),
        reseaon,
        url
    })));
}

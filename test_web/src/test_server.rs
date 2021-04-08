use crate::response::ResponseBody;
use actix_web::{post, web, HttpResponse, Responder};
//use futures::StreamExt;
use log::{warn};
use serde::{Deserialize, Serialize};


//webhook数据通知类型
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookReqwest {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    created: i64,
    event: String,
    data: serde_json::Value
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthReqwest{
    version: u32,          
    payload: String,     
    nonce: String        
}

#[post("/test/webhook")]
 pub async fn test_server_info(
    req: web::Json<AuthReqwest>
 )-> impl Responder {
    warn!("Chunk: {:?}", req);
    return HttpResponse::Ok().json(ResponseBody::<()>::new_success(None));
 }
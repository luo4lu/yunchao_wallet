use actix_web::{App, HttpServer, web};
use log::Level;
use std::fs::File;
use std::io::BufReader;

mod config;
mod register_b;
pub mod response;
pub mod authen_info;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Initialize the log and set the print level
    simple_logger::init_with_level(Level::Warn).unwrap();
    //配置数据库
    let file = File::open("./config/config_info.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let pay_addr: String = value["server_addr"].as_str().unwrap().to_string();
    HttpServer::new(|| {
        App::new()
            .app_data(web::PayloadConfig::new(1000000 * 250))
            .app_data(web::JsonConfig::default().limit(1000000 * 250))
            .service(register_b::user_info_save)
    })
    .bind(pay_addr)?
    .run()
    .await
}

use actix_web::{App, HttpServer};
use log::Level;
use std::fs::File;
use std::io::BufReader;

mod config;
mod transfer;
mod get_info;
pub mod response;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Initialize the log and set the print level
    simple_logger::init_with_level(Level::Warn).unwrap();
    //配置数据库
    let file = File::open("./config/config.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let pay_addr: String = value["payment_addr"].as_str().unwrap().to_string();
    HttpServer::new(|| {
        App::new()
            .data(config::get_db())
            .service(transfer::create_payment)
            .service(get_info::get_trans_info)
            .service(get_info::get_trans_list)
    })
    .bind(pay_addr)?
    .run()
    .await
}
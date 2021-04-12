use actix_web::{App, HttpServer};
use log::Level;
use std::fs::File;
use std::io::BufReader;

mod config;
mod url_status;
mod get_status;
pub mod response;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Initialize the log and set the print level
    simple_logger::init_with_level(Level::Warn).unwrap();
    //配置数据库
    let file = File::open("./config/config_user.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let pay_addr: String = value["server_addr"].as_str().unwrap().to_string();
    
    HttpServer::new(|| {
        App::new()
            .service(url_status::url_statu_save)
            .service(get_status::get_info)
    })
    .bind(pay_addr)?
    .run()
    .await
}

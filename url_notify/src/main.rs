use actix_web::{App, HttpServer};
use log::Level;
use std::fs::File;
use std::io::BufReader;

mod send_url;
mod recv_url;
mod config;
pub mod response;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Initialize the log and set the print level
    simple_logger::init_with_level(Level::Warn).unwrap();

    let file = File::open("./config/config.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let notify_addr: String = value["notify_addr"].as_str().unwrap().to_string();
    HttpServer::new(|| {
        App::new()
            .service(send_url::get_url)
            .service(recv_url::user_notify)
            .service(send_url::delete_url)
    })
    .bind(notify_addr)?
    .run()
    .await
}
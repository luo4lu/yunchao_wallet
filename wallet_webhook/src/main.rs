//use actix_web::{App, HttpServer};
//use clap::ArgMatches;
//use log::Level;
//use actix_cors::Cors; //跨域crate
use crate::consumer::consumer_server;
mod consumer;
pub mod config;
//pub mod data_struct;

#[tokio::main]
async fn main() {
    //simple_logger::init_with_level(Level::Info).unwrap();
    env_logger::init();
    consumer_server().await;
}
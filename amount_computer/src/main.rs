use log::Level;

mod get_amount;
mod config;
pub mod response;

#[tokio::main]
async fn main() {
    //Initialize the log and set the print level
    simple_logger::init_with_level(Level::Info).unwrap();
    get_amount::get_day_amount().await;
    return 
}


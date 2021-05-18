use mysql_async::{Pool, OptsBuilder};
use std::fs::File;
use std::io::BufReader;



pub fn get_db() -> Pool {
    //配置数据库
    let file = File::open("./config/config.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let dbaddr: String = value["dbaddr"].as_str().unwrap().to_string();
    let dbname: String = value["dbname"].as_str().unwrap().to_string();
    let dbbase: String = value["dbbase"].as_str().unwrap().to_string();
    let dbpass: String = value["dbpass"].as_str().unwrap().to_string();
    let port: String = value["port"].as_str().unwrap().to_string();
    let dbport: u16 = port.parse::<u16>().unwrap();
    let existing_opts = OptsBuilder::default()
                        .ip_or_hostname(dbaddr)
                        .tcp_port(dbport)
                        .user(Some(dbname))
                        .pass(Some(dbpass))
                        .db_name(Some(dbbase));
    let pool = Pool::new(existing_opts);
    return pool; 
}
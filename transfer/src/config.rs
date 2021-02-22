use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};
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
    let mut cfg = Config::new();
    cfg.host(&dbaddr); //数据库地址
    cfg.user(&dbname); //数据库用户名
    cfg.password("postgres"); //数据库密码
    cfg.dbname(&dbbase); //数据库名称
    let mgr = Manager::new(cfg, NoTls); //生成一个数据库管理池
    Pool::new(mgr, 8) //设置管理池最大连接数并返回池对象
}

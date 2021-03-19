use test_db;
create table user_info(
    appid varchar(255) primary key not null ,
    secret_key varchar(255) not null ,
    web_url text not null ,
    create_time timestamp not null DEFAULT CURRENT_TIMESTAMP,
    update_time timestamp null 
);
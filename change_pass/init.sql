use test_db;
create table url_statu(
    user_id varchar(255) primary key not null ,
    url text not null ,
    status boolean not null,
    create_time timestamp not null DEFAULT CURRENT_TIMESTAMP,
    update_time timestamp null 
);
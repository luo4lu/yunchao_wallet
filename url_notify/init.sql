create table url_notify(
    wallet_id varchar(255) primary key not null ,
    status varchar(50) not null ,
    reseaon text,
    url text,
    create_time timestamp not null DEFAULT CURRENT_TIMESTAMP,
);
use test_db;
create table business(
    id varchar(255) primary key not null ,
    bus_type varchar(255) not null ,
    bus_name varchar(255) not null,
    credit_code varchar(255) not null,
    validity_begin bigint,
    validity_end bigint not null,
    reg_addr varchar(255) not null,
    bus_addr varchar(255) not null,
    bus_info varchar(255) not null,
    reg_capital bigint,
    linkman varchar(10) not null,
    telephone varchar(20) not null,
    link_email varchar(50) not null,
    account_note1 varchar(255),
    account_note2 varchar(255),
    account_note3 varchar(255),
    bus_connect varchar(255) not null,
    person_info json not null,
    bank_info json not null,
    create_time timestamp not null DEFAULT CURRENT_TIMESTAMP
);
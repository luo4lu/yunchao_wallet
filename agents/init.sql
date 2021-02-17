create table agents(
    id varchar(255) PRIMARY KEY NOT NULL,
    type varchar(255) NOT NULL,
    created timestamp NOT NULL,
    from_wallet varchar(255) not null,
    to_wallet varchar(255) NOT NULL,
    begin_time timestamp NOT NULL,
    end_time timestamp NOT NULL,
    limit_amount bigint NOT NULL,
    day_limit_amount bigint,
    month_limit_amount bigint,
    total_limit_amount bigint,
    description varchar(255),
    extra jsonb,
	update_time timestamp
);
create table transfer(
    id varchar(255) PRIMARY KEY NOT NULL,
    type varchar(255) NOT NULL,
    created timestamp NOT NULL,
	extra jsonb,
	wallet_id VARCHAR(255) NOT NULL,
	to_wallet VARCHAR(255) NOT NULL,
    description VARCHAR(255),
	status VARCHAR(255) not NULL,
	update_time timestamp
);
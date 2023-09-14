create table users(
    id int unsigned auto_increment primary key,
    name varchar(50) not null,
    email varchar(50) not null unique,
    password_hash varchar(100) not null,
    verification_token char(36) not null unique,
    verified boolean not null,
    refresh_token varchar(32)
);

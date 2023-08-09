create table users(
    id bigserial primary key,
    name varchar(50) not null,
    email varchar(50) not null unique,
    password_hash varchar(100) not null,
    verification_token uuid not null unique,
    verified boolean not null,
    refresh_token varchar(32)
);

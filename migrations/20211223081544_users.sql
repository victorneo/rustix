-- Add migration script here
CREATE TABLE users (
    id          integer PRIMARY KEY,
    email       varchar(255) unique not null,
    password    varchar(255) not null,
    first_name varchar(50) null,
    last_name varchar(50) null,
    active boolean default true
)
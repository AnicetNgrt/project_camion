-- Add up migration script here

CREATE TYPE user_role AS ENUM ('admin', 'author', 'none');

CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    role user_role NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL
)
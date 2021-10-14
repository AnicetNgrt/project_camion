-- Add up migration script here

CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    role INTEGER NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL
)
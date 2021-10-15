-- Add up migration script here

CREATE TABLE jokes (
    id SERIAL PRIMARY KEY NOT NULL,
    title VARCHAR NOT NULL,
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL,
    modified_at TIMESTAMP NOT NULL
)
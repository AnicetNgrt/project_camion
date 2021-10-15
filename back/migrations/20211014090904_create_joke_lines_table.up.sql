-- Add up migration script here

CREATE TABLE joke_lines (
    id SERIAL PRIMARY KEY NOT NULL,
    speaker VARCHAR NOT NULL,
    content TEXT NOT NULL,
    index_within_joke INTEGER NOT NULL,
    joke_id INTEGER NOT NULL REFERENCES jokes(id) ON DELETE CASCADE
)
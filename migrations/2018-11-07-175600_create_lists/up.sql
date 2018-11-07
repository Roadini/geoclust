-- Your SQL goes here

CREATE TABLE lists (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    list_name VARCHAR NOT NULL
)

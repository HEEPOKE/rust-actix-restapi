CREATE TABLE shops (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    address TEXT NOT NULL,
    telephone VARCHAR,
    user_id INT4 NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
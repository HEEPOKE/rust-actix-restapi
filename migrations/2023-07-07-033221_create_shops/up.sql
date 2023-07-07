CREATE TABLE shops (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    address TEXT NOT NULL,
    telephone VARCHAR,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

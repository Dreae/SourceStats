CREATE TABLE accounts (
    user_id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    username VARCHAR(128) NOT NULL,
    password CHAR(128) NOT NULL,
    UNIQUE(username),
    UNIQUE(email)
);
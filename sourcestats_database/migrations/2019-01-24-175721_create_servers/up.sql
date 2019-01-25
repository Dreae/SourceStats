CREATE TABLE servers (
    server_id SERIAL PRIMARY KEY,
    server_name TEXT NOT NULL,
    server_address CHAR(24) NOT NULL,
    server_website TEXT
);
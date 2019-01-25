CREATE TABLE server_keys (
    key_id BIGSERIAL PRIMARY KEY,
    key_data BYTEA NOT NULL,
    server_id INTEGER NOT NULL REFERENCES servers(server_id)
);
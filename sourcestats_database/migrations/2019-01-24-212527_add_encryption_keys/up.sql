CREATE TABLE server_keys (
    key_id BIGINT PRIMARY KEY,
    key_data BYTEA NOT NULL,
    server_id BIGINT NOT NULL REFERENCES servers(server_id)
);
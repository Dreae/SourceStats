CREATE TABLE kills (
    time TIMESTAMP NOT NULL,
    server_id INTEGER REFERENCES servers(server_id),
    map VARCHAR(128) NOT NULL,
    pos_x INTEGER NOT NULL,
    pos_y INTEGER NOT NULL,
    pos_z INTEGER NOT NULL,
    steam_id NUMERIC NOT NULL,
    other_steam_id NUMERIC NOT NULL,
    headshot BIT NOT NULL DEFAULT 0::bit,
    weapon SMALLINT NOT NULL
);

SELECT create_hypertable('kills', 'time', 'server_id', 16);
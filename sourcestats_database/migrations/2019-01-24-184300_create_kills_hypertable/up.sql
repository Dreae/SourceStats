CREATE TABLE kills (
    time TIMESTAMPTZ NOT NULL,
    server_id INTEGER REFERENCES servers(server_id),
    map VARCHAR(128) NOT NULL,
    pos_x INTEGER NOT NULL,
    pos_y INTEGER NOT NULL,
    pos_z INTEGER NOT NULL,
    killer_id INTEGER NOT NULL REFERENCES players(player_id),
    victim_id INTEGER NOT NULL REFERENCES players(player_id),
    headshot BIT NOT NULL DEFAULT 0::bit,
    weapon SMALLINT NOT NULL
);

SELECT create_hypertable('kills', 'time', 'server_id', 8);
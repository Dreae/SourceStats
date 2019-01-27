CREATE TABLE rank_history (
    time TIMESTAMPTZ NOT NULL,
    server_id BIGINT NOT NULL REFERENCES servers(server_id),
    player_id BIGINT NOT NULL REFERENCES players(player_id),
    rank INTEGER NOT NULL DEFAULT 2000
);

CREATE INDEX idx_rank_player_id ON rank_history(player_id, time DESC);
CREATE INDEX idx_rank_server_id ON rank_history(server_id, time DESC);
CREATE INDEX idx_rank_server_player_query ON rank_history(time DESC, player_id, server_id);

SELECT create_hypertable('rank_history', 'time');
CREATE TABLE players (
    player_id BIGSERIAL PRIMARY KEY,
    steam_id BIGINT UNIQUE NOT NULL
);
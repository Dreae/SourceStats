CREATE TABLE players (
    player_id SERIAL PRIMARY KEY,
    steam_id NUMERIC UNIQUE NOT NULL
);
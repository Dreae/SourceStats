use diesel::prelude::*;
use diesel::result::Error;
use crate::schema::players::dsl::*;

#[derive(Queryable)]
pub struct Player {
    pub player_id: i64,
    pub steam_id: i64,
}

impl Player {
    pub fn get_by_steam_id(id: i64, conn: &PgConnection) -> Result<Player, Error> {
        players.filter(steam_id.eq(id)).get_result(conn)
    }

    pub fn get_by_id(id: i64, conn: &PgConnection) -> Result<Player, Error> {
        players.filter(player_id.eq(id)).get_result(conn)
    }
}
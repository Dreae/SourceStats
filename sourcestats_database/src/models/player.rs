use diesel::prelude::*;
use diesel::result::Error;
use diesel;
use crate::schema::players;

#[derive(Queryable)]
pub struct Player {
    pub player_id: i64,
    pub steam_id: i64,
}

#[derive(Insertable)]
#[table_name="players"]
struct NewPlayer {
    steam_id: i64,
}

impl Player {
    pub fn insert(new_steam_id: i64, conn: &PgConnection) -> Result<Player, Error> {
        let new_player = NewPlayer {
            steam_id: new_steam_id
        };

        diesel::insert_into(players::table).values(&new_player).get_result(conn)
    }

    pub fn get_by_steam_id(id: i64, conn: &PgConnection) -> Result<Player, Error> {
        players::table.filter(players::steam_id.eq(id)).get_result(conn)
    }

    pub fn get_by_id(id: i64, conn: &PgConnection) -> Result<Player, Error> {
        players::table.filter(players::player_id.eq(id)).get_result(conn)
    }
}
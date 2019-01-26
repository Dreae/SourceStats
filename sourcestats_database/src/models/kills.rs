use chrono::{Utc, DateTime};
use diesel::result::Error;
use diesel::sql_types::*;
use diesel::prelude::*;
use diesel;

pub struct Kill {
    pub timestamp: DateTime<Utc>,
    pub map: String,
    pub server_id: i64,
    pub killer_id: i64,
    pub victim_id: i64,
    pub headshot: bool,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub weapon: i16
}

impl Kill {
    pub fn save(&self, conn: &PgConnection) -> Result<usize, Error> {
        diesel::sql_query(
            "INSERT INTO kills (\
                time,\
                server_id,\
                map,\
                killer_id,\
                victim_id,\
                headshot,\
                pos_x,\
                pos_y,\
                pos_z,\
                weapon\
            ) VALUES (\
                ?,\
                ?,\
                ?,\
                ?,\
                ?,\
                ?,\
                ?,\
                ?,\
                ?,\
                ?\
            )"
        )
            .bind::<Timestamptz, _>(self.timestamp)
            .bind::<BigInt, _>(self.server_id)
            .bind::<Text, _>(&self.map)
            .bind::<BigInt, _>(self.killer_id)
            .bind::<BigInt, _>(self.victim_id)
            .bind::<Bool, _>(self.headshot)
            .bind::<Integer, _>(self.pos_x)
            .bind::<Integer, _>(self.pos_y)
            .bind::<Integer, _>(self.pos_z)
            .bind::<SmallInt, _>(self.weapon)
            .execute(conn)
    }
}
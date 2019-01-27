use capnp::message::{Reader, Builder};
use capnp::serialize::OwnedSegments;
use capnp::serialize_packed;
use capnp::Result;
use bytes::{BytesMut, BufMut};

use crate::protocol_capnp::player_update;
use crate::CapnpSerialize;

pub struct PlayerUpdate {
    pub steam_id: i64,
    pub kills: Vec<InGameKill>,
    pub shots: Vec<ShotFired>,
}

pub struct ShotFired {
    pub timestamp: i64,
    pub hit: bool,
}

pub struct InGameKill {
    pub timestamp: i64,
    pub victim_id: i64,
    pub map: String,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub weapon: i16,
    pub headshot: bool,
}

impl PlayerUpdate {
    pub fn from_capnp(reader: Reader<OwnedSegments>) -> Result<PlayerUpdate> {
        let update = reader.get_root::<player_update::Reader>()?;
        let mut kills = Vec::new();
        for kill in update.get_kills()? {
            let in_game_kill = InGameKill {
                timestamp: kill.get_timestamp(),
                victim_id: kill.get_other(),
                map: kill.get_map()?.to_owned(),
                pos_x: kill.get_pos_x(),
                pos_y: kill.get_pos_y(),
                pos_z: kill.get_pos_z(),
                weapon: kill.get_weapon(),
                headshot: kill.get_headshot()
            };

            kills.push(in_game_kill);
        }

        let shots = update.get_shots()?.iter().map(|shot| {
            ShotFired {
                timestamp: shot.get_timestamp(),
                hit: shot.get_hit(),
            }
        }).collect();

        Ok(PlayerUpdate{
            steam_id: update.get_steam_id64(),
            kills,
            shots,
        })
    }
}

impl CapnpSerialize for PlayerUpdate {
    fn serialize(self) -> Result<BytesMut> {
        let mut builder = Builder::new_default();
        let mut update = builder.init_root::<player_update::Builder>();
        update.set_steam_id64(self.steam_id);

        {
            let mut kills = update.reborrow().init_kills(self.kills.len() as u32);

            for i in 0..self.kills.len() {
                let kill = self.kills.get(i).unwrap();
                let mut kill_builder = kills.reborrow().get(i as u32);
                kill_builder.set_timestamp(kill.timestamp);
                kill_builder.set_other(kill.victim_id);
            }
        }

        {
            let mut shots = update.reborrow().init_shots(self.shots.len() as u32);

            for i in 0..self.shots.len() {
                let shot = self.shots.get(i).unwrap();
                let mut shot_builder = shots.reborrow().get(i as u32);
                shot_builder.set_timestamp(shot.timestamp);
                shot_builder.set_hit(shot.hit);
            }
        }

        let mut buf = BytesMut::with_capacity(1024).writer();
        serialize_packed::write_message(&mut buf, &builder)?;

        Ok(buf.into_inner())
    }
}
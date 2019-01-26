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
}

impl PlayerUpdate {
    pub fn from_capnp(reader: Reader<OwnedSegments>) -> Result<PlayerUpdate> {
        let update = reader.get_root::<player_update::Reader>()?;
        let kills = update.get_kills()?.iter().map(|kill| {
            InGameKill {
                timestamp: kill.get_timestamp(),
                victim_id: kill.get_other(),
            }
        }).collect();

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
            let mut kills = update.init_kills(self.kills.len() as u32);

            for i in 0..self.kills.len() {
                let kill = self.kills.get(i).unwrap();
                let mut kill_builder = kills.reborrow().get(i as u32);
                kill_builder.set_timestamp(kill.timestamp);
                kill_builder.set_other(kill.victim_id);
            }
        }

        let mut buf = BytesMut::with_capacity(1024).writer();
        serialize_packed::write_message(&mut buf, &builder)?;

        Ok(buf.into_inner())
    }
}
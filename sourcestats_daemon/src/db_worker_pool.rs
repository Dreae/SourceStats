use crate::packet::Packet;
use mio::{Ready, Registration, Poll, PollOpt, Token, Evented, SetReadiness};
use rayon::{ThreadPoolBuilder, ThreadPool};
use sourcestats_database::{Pool, ServerKey, SQLError, Kill, Player};
use sourcestats_protocol::{Message, DecryptError};
use chrono::prelude::*;
use crossbeam::channel::{self as mpmc, Sender, Receiver, TryRecvError};

use std::sync::Arc;
use std::io;

pub struct DbWorkerService {
    reply_sender: Sender<Packet>,
    reply_receiver: Receiver<Packet>,
    registration: Arc<Registration>,
    set_readiness: Arc<SetReadiness>,
    thread_pool: ThreadPool,
    pool: Pool,
}

#[derive(Debug)]
enum ProcessingError {
    SqlError(SQLError),
    ParseError(DecryptError),
}

impl From<SQLError> for ProcessingError {
    fn from(err: SQLError) -> Self {
        ProcessingError::SqlError(err)
    }
}

impl From<DecryptError> for ProcessingError {
    fn from(err: DecryptError) -> Self {
        ProcessingError::ParseError(err)
    }
}

type ProcessingResult = Result<(), ProcessingError>;

impl DbWorkerService {
    pub fn new(threads: usize, pool: Pool) -> DbWorkerService {
        let (reply_send, reply_recv) = mpmc::unbounded();
        let (registration, set_readiness) = Registration::new2();
        let thread_pool = ThreadPoolBuilder::new().num_threads(threads).thread_name(|id| {
            format!("DbWorker-{}", id)
        }).panic_handler(|err| error!("Worker pool panic! {:?}", err))
            .build().expect("Error building DbWorker thread pool");

        DbWorkerService {
            reply_sender: reply_send,
            reply_receiver: reply_recv,
            registration: Arc::new(registration),
            set_readiness: Arc::new(set_readiness),
            thread_pool,
            pool,
        }
    }

    pub fn submit(&self, packet: Packet) {
        let pool = self.pool.clone();
        // TODO: Error handling
        self.thread_pool.spawn(move || {
            if let Err(e) = DbWorkerService::parse_packet(packet, pool) {
                error!("Error processing message {:?}", e)
            }
        });
    }

    pub fn get_reply(&self) -> Result<Packet, io::Error> {
        match self.reply_receiver.try_recv() {
            Ok(packet) => Ok(packet),
            Err(TryRecvError::Empty) => {
                match self.set_readiness.set_readiness(Ready::empty()) {
                    Err(e) => error!("Error clearing thread pool ready state {}", e),
                    _ => { },
                };
                Err(io::ErrorKind::WouldBlock.into())
            },
            Err(TryRecvError::Disconnected) => Err(io::ErrorKind::Other.into()),
        }
    }

    fn parse_packet(packet: Packet, pool: Pool) -> ProcessingResult {
        let connection = pool.get_connection();
        let key =  ServerKey::get_by_id(packet.key_id, &connection)?;
        let message = Message::decrypt(&key.key_data, packet.nonce, packet.data)?;
        match message {
            Message::PlayerUpdate(update) => {
                let player = match Player::get_by_steam_id(update.steam_id, &connection) {
                    Ok(player) => player,
                    Err(SQLError::NotFound) => Player::insert(update.steam_id, &connection)?,
                    Err(e) => return Err(e.into()),
                };

                for kill in update.kills.into_iter() {
                    let victim = match Player::get_by_steam_id(kill.victim_id, &connection) {
                        Ok(victim) => victim,
                        Err(SQLError::NotFound) => Player::insert(kill.victim_id, &connection)?,
                        Err(e) => return Err(e.into()),
                    };

                    let db_kill = Kill {
                        timestamp: Utc.timestamp_millis(kill.timestamp),
                        map: kill.map,
                        server_id: key.server_id,
                        victim_id: victim.player_id,
                        killer_id: player.player_id,
                        headshot: kill.headshot,
                        pos_x: kill.pos_x,
                        pos_y: kill.pos_y,
                        pos_z: kill.pos_z,
                        weapon: kill.weapon
                    };

                    if let Err(e) = db_kill.save(&connection) {
                        error!("Error recording kill: {}", e);
                    };
                }
            }
        };


        Ok(())
    }
}

impl Evented for DbWorkerService {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
        self.registration.register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
        self.registration.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        self.registration.deregister(poll)
    }
}
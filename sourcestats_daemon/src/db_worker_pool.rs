use crate::packet::Packet;
use mio::{Ready, Registration, Poll, PollOpt, Token, Evented, SetReadiness};
use rayon::{ThreadPoolBuilder, ThreadPool};
use sourcestats_database::{Pool, ServerKey};
use sourcestats_protocol::Message;

use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
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

impl DbWorkerService {
    pub fn new(threads: usize, pool: Pool) -> DbWorkerService {
        let (reply_send, reply_recv) = mpsc::channel();
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
        let connection = self.pool.clone().get_connection();
        // TODO: Error handling
        self.thread_pool.spawn(move || {
            match ServerKey::get_by_id(packet.key_id, &connection) {
                Ok(key) => {
                    let _message = Message::decrypt(&key.key_data, packet.nonce, packet.data);
                },
                Err(e) => error!("Error getting server details {}", e),
            };
        })
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
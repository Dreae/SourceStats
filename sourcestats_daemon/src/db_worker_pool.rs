use crate::packet::Packet;
use mio::{Ready, Registration, Poll, PollOpt, Token, Evented, SetReadiness};
use rayon::{ThreadPoolBuilder, ThreadPool};

use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use std::sync::Arc;
use std::io;

pub struct DbWorkerService {
    reply_sender: Sender<Packet>,
    reply_receiver: Receiver<Packet>,
    registration: Arc<Registration>,
    set_readiness: Arc<SetReadiness>,
    thread_pool: ThreadPool,
}

impl DbWorkerService {
    pub fn new(threads: usize) -> DbWorkerService {
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
        }
    }

    pub fn submit(&self, packet: Packet) {
        let sender = self.reply_sender.clone();
        let set_readiness = self.set_readiness.clone();
        self.thread_pool.spawn(move || {
            sender.send(packet).expect("Work pool shutdown"); // Echo the packet back
            match set_readiness.set_readiness(Ready::readable()) { // Notify the worker pool there's something to read
                Err(e) => error!("Error notifying thread pool of reply: {}", e),
                _ => { }
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
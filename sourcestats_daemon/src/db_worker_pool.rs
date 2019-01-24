use crossbeam_channel::{self as mpmc, Sender, Receiver};
use crate::packet::Packet;
use mio::{Ready, Registration, Poll, PollOpt, Token, Evented, SetReadiness};

use std::sync::mpsc::{self, Sender as MPSCSender, Receiver as MPSCReceiver, TryRecvError};
use std::thread::Builder;
use std::sync::Arc;
use std::io;

pub struct DbWorkerService {
    packet_sender: Sender<Packet>,
    reply_receiver: MPSCReceiver<Packet>,
    registration: Arc<Registration>,
    set_readiness: Arc<SetReadiness>,
}

struct DbWorker {
    id: usize,
    packet_receiver: Receiver<Packet>,
    reply_sender: MPSCSender<Packet>,
    set_readiness: SetReadiness,
}

impl DbWorkerService {
    pub fn new(threads: usize) -> DbWorkerService {
        let (send, recv) = mpmc::unbounded();
        let (reply_send, reply_recv) = mpsc::channel();
        let (registration, set_readiness) = Registration::new2();

        for id in 0..threads {
            DbWorker::new(id, recv.clone(), reply_send.clone(), set_readiness.clone());

        }

        DbWorkerService {
            packet_sender: send,
            reply_receiver: reply_recv,
            registration: Arc::new(registration),
            set_readiness: Arc::new(set_readiness),
        }
    }

    pub fn submit(&self, packet: Packet) {
        match self.packet_sender.send(packet) {
            Err(_) => error!("Error submitting packet, pool is shutdown"),
            _ => { }
        };
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

impl DbWorker {
    pub fn new(id: usize, receiver: Receiver<Packet>, reply_sender: MPSCSender<Packet>, set_readiness: SetReadiness) {
        let worker = DbWorker {
            id,
            packet_receiver: receiver,
            reply_sender,
            set_readiness,
        };

        worker.start();
    }

    fn start(self) {
        Builder::new().name(format!("DbPoolThread-{}", self.id)).spawn(move || {
            for packet in self.packet_receiver.iter() {
                trace!("DbPoolThread-{} received new packet", self.id);
                self.reply_sender.send(packet).expect("Work pool shutdown"); // Echo the packet back
                match self.set_readiness.set_readiness(Ready::readable()) { // Notify the worker pool there's something to read
                    Err(e) => error!("Error notifying thread pool of reply: {}", e),
                    _ => { }
                };
            }
        }).unwrap();
    }
}
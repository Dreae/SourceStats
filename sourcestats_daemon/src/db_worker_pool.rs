use crossbeam_channel::{self as mpmc, Sender, Receiver};
use crate::packet::Packet;

use std::thread::Builder;

pub struct DbWorkerService {
    packet_sender: Sender<Packet>,
}

struct DbWorker {
    id: usize,
    packet_receiver: Receiver<Packet>,
}

impl DbWorkerService {
    pub fn new(threads: usize) -> DbWorkerService {
        let (send, recv) = mpmc::unbounded();

        for id in 0..threads {
            DbWorker::new(id, recv.clone());
        }

        DbWorkerService {
            packet_sender: send,
        }
    }

    pub fn submit(&self, packet: Packet) {
        match self.packet_sender.send(packet) {
            Err(_) => error!("Error submitting packet, pool is shutdown"),
            _ => { }
        };
    }
}

impl Clone for DbWorkerService {
    fn clone(&self) -> Self {
        DbWorkerService {
            packet_sender: self.packet_sender.clone()
        }
    }
}

impl DbWorker {
    pub fn new(id: usize, receiver: Receiver<Packet>) {
        let worker = DbWorker {
            id,
            packet_receiver: receiver,
        };

        worker.start();
    }

    fn start(self) {
        Builder::new().name(format!("DbPoolThread-{}", self.id)).spawn(move || {
            for packet in self.packet_receiver.iter() {
                trace!("DbPoolThread-{} received new packet", self.id);
            }
        }).unwrap();
    }
}
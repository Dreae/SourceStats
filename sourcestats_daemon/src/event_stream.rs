use mio::net::TcpStream;
use bytes::BytesMut;
use mio::{Poll, Ready, PollOpt, Token};
use byteorder::{ByteOrder, NetworkEndian};
use crate::packet::Packet;
use crate::db_worker_pool::DbWorkerService;

use std::io::{Write, Read, ErrorKind};
use std::rc::Rc;
use std::io;

pub struct EventStream {
    pub stream: TcpStream,
    pub token: Token,
    event_loop: Rc<Poll>,
    pub interest: Ready,
    read_buf: [u8; 4096],
    rx: BytesMut,
    tx: BytesMut,
    work_pool: Rc<DbWorkerService>,
}

impl EventStream {
    pub fn new(event_loop: Rc<Poll>, stream: TcpStream, work_pool: Rc<DbWorkerService>) -> EventStream {
        let mut interest = Ready::all();
        interest.remove(Ready::writable());

        EventStream {
            stream,
            token: Token(0),
            event_loop,
            interest,
            read_buf: [0u8; 4096],
            rx: BytesMut::with_capacity(4096),
            tx: BytesMut::with_capacity(4096),
            work_pool,
        }
    }

    pub fn readable(&mut self) -> Result<(), io::Error> {
        loop {
            match self.stream.read(&mut self.read_buf) {
                Ok(bytes_read) => {
                    trace!("Read {} bytes", bytes_read);
                    if bytes_read == 0 {
                        trace!("Got EOF, closing socket {}", Into::<usize>::into(self.token));
                        return Err(ErrorKind::ConnectionReset.into());
                    }

                    self.rx.extend_from_slice(&self.read_buf[..bytes_read]);
                },
                Err(e) => {
                    if e.kind() != ErrorKind::WouldBlock {
                        return Err(e);
                    }

                    break;
                },
            };
        }

        while self.rx.len() > 2 {
            let msg_len = NetworkEndian::read_u16(&self.rx) as usize;
            trace!("Expecting message of size {} current bytes read {}", msg_len, self.rx.len());
            if msg_len < 24 {
                error!("{:?}: Message length is too short to be a well-formed message", self.token);
                warn!("{:?}: Clearing corrupted buffer from {:?}", self.token, self.stream.peer_addr());
                self.rx.clear();
            } else if self.rx.len() - 2 >= msg_len {
                trace!("Submitting new packet to workpool");
                self.work_pool.submit(Packet::from_buf(self.token, self.rx.split_to(msg_len)));
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn writable(&mut self) -> Result<(), io::Error> {
        if self.tx.len() > 0 {
            let bytes_written = self.stream.write(&self.tx)?;
            trace!("Wrote {} bytes", bytes_written);

            self.tx.advance(bytes_written);
        }

        if self.tx.len() == 0 {
            trace!("Wrote all bytes from buffer for {:?}", self.token);
            self.interest.remove(Ready::writable());
            self.reregister();
        }

        Ok(())
    }

    pub fn reregister(&self) {
        trace!("Registering socket {:?} with interest {:?}", self.token, self.interest);
        match self.event_loop.reregister(&self.stream, self.token, self.interest, PollOpt::edge()) {
            Err(e) => error!("Error reregistering interest for {:?}: {}", self.token, e),
            _ => { }
        };
    }

    pub fn write(&mut self, buf: BytesMut) {
        self.tx.unsplit(buf);
        self.interest.insert(Ready::writable());
        self.reregister();
    }
}
use mio::net::TcpStream;
use bytes::BytesMut;
use std::io::{Write, Read, ErrorKind};
use std::rc::Rc;
use mio::{Poll, Ready, PollOpt, Token};
use std::io;

pub struct EventStream {
    pub stream: TcpStream,
    pub token: Token,
    event_loop: Rc<Poll>,
    interest: Ready,
    read_buf: [u8; 4096],
    rx: BytesMut,
    tx: BytesMut,
}

impl EventStream {
    pub fn new(event_loop: Rc<Poll>, stream: TcpStream) -> EventStream {
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
        }
    }

    pub fn readable(&mut self) -> Result<(), io::Error> {
        let bytes_read = self.stream.read(&mut self.read_buf)?;
        trace!("Read {} bytes", bytes_read);
        if bytes_read == 0 {
            trace!("Got EOF, closing socket {}", Into::<usize>::into(self.token));
            return Err(Into::<io::Error>::into(ErrorKind::ConnectionReset));
        }

        self.rx.extend_from_slice(&self.read_buf[..bytes_read]);

        let buf = self.rx.take();
        self.write(&buf[..]);

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

    pub fn write(&mut self, buf: &[u8]) {
        self.tx.extend_from_slice(buf);
        self.interest.insert(Ready::writable());
        self.reregister();
    }
}
use mio::net::TcpStream;
use bytes::BytesMut;
use std::io::{Write, Read};
use std::io;

pub struct EventStream {
    pub stream: TcpStream,
    read_buf: [u8; 4096],
    rx: BytesMut,
    tx: BytesMut,
}

impl EventStream {
    pub fn new(stream: TcpStream) -> EventStream {
        EventStream {
            stream,
            read_buf: [0u8; 4096],
            rx: BytesMut::with_capacity(4096),
            tx: BytesMut::with_capacity(4096),
        }
    }

    pub fn readable(&mut self) -> Result<(), io::Error> {
        let bytes_read = self.stream.read(&mut self.read_buf)?;
        trace!("Read {} bytes", bytes_read);

        self.rx.extend_from_slice(&self.read_buf[..bytes_read]);
        self.tx.extend_from_slice(&self.rx.take()[..]);

        Ok(())
    }

    pub fn writable(&mut self) -> Result<(), io::Error> {
        if self.tx.len() > 0 {
            let bytes_written = self.stream.write(&self.tx)?;
            trace!("Wrote {} bytes", bytes_written);

            self.tx.advance(bytes_written);
        }

        Ok(())
    }
}
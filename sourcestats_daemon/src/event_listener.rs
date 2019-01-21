use std::net::SocketAddr;
use std::io;
use std::u32;

use failure::Fallible;
use mio::{Token, Ready, PollOpt, Poll, Events, Event};
use mio::net::TcpListener;
use slab::Slab;

use crate::event_stream::EventStream;

pub struct EventListener {
    addr: SocketAddr,
    connections: Slab<EventStream>,
    event_loop: Poll,
}

impl EventListener {
    pub fn new(addr: &str) -> Fallible<EventListener> {
        Ok(EventListener {
            addr: addr.parse().expect(&format!("Invalid address {}", addr)),
            connections: Slab::new(),
            event_loop: Poll::new()?,
        })
    }

    pub fn listen(&mut self) -> Fallible<()> {
        const SERVER: Token = Token((u32::MAX - 1) as usize);
        let server = TcpListener::bind(&self.addr)?;
        self.event_loop.register(&server, SERVER, Ready::readable(), PollOpt::edge())?;

        let mut events = Events::with_capacity(1024);

        loop {
            self.event_loop.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        trace!("New TCP connection");
                        let socket = server.accept();
                        match socket {
                            Ok((stream, _addr)) => {
                                debug!("Accepted new connection from {}", _addr);
                                let tok = self.connections.insert(EventStream::new(stream));
                                match self.event_loop.register(&self.connections[tok].stream, Token(tok), Ready::all(), PollOpt::edge()) {
                                    Err(e) => error!("Could no register socket with event loop {}", e),
                                    _ => { }
                                };
                            },
                            Err(e) => {
                                error!("TCP Accept error: {}", e);
                            }
                        }
                    },
                    Token(tok) => {
                        let result = if self.connections.contains(tok) {
                            let stream = &mut self.connections[tok];

                            EventListener::process_event(stream, &event)
                        } else {
                            Ok(())
                        };

                        self.check_error(tok, result);
                    }
                }
            }
        }
    }

    fn process_event(stream: &mut EventStream, event: &Event) -> Result<(), io::Error> {
        if event.readiness().contains(Ready::readable()) {
            trace!("Socket {} is readable", Into::<usize>::into(event.token()));
            stream.readable()?;
        }

        if event.readiness().contains(Ready::writable()) {
            trace!("Socket {} is writable", Into::<usize>::into(event.token()));
            stream.writable()?;
        }

        Ok(())
    }

    fn check_error(&mut self, token: usize, result: Result<(), io::Error>) {
        if result.is_err() {
            let err = result.err().unwrap();
            match err.kind() {
                io::ErrorKind::ConnectionReset |
                io::ErrorKind::ConnectionAborted |
                io::ErrorKind::BrokenPipe => {
                    debug!("Got close error, removing {} from pool", token);
                    self.connections.remove(token);
                }
                io::ErrorKind::WouldBlock => { },
                _ => error!("Socket error: {}", err)
            };
        };
    }
}
use std::net::SocketAddr;

use failure::Fallible;
use mio::{Token, Ready, PollOpt, Poll, Events};
use mio::net::TcpListener;
use slab::Slab;

use crate::event_stream::EventStream;

pub struct EventListener {
    addr: SocketAddr,
    connections: Slab<EventStream>,
}

impl EventListener {
    pub fn new(addr: &str) -> Fallible<EventListener> {
        Ok(EventListener {
            addr: addr.parse().expect(&format!("Invalid address {}", addr)),
            connections: Slab::new(),
        })
    }

    pub fn listen(&mut self) -> Fallible<()> {
        const SERVER: Token = Token(0);
        let server = TcpListener::bind(&self.addr)?;
        let poll = Poll::new()?;
        poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())?;

        let mut events = Events::with_capacity(1024);

        loop {
            poll.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        trace!("New TCP connection");
                        let socket = server.accept();
                        match socket {
                            Ok((stream, _addr)) => {
                                let tok = self.connections.insert(EventStream::new(stream));
                                match poll.register(&self.connections[tok].stream, Token(tok), Ready::readable(), PollOpt::edge()) {
                                    Err(e) => error!("Could no register socket with event loop {}", e),
                                    _ => { }
                                };
                            },
                            Err(e) => {
                                error!("TCP Accept error: {}", e);
                            }
                        }
                    }
                    Token(tok) => {
                        self.connections.get(tok).map(|stream| {

                        });
                    }
                }
            }
        }
    }
}
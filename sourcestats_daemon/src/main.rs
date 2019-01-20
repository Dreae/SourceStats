extern crate capnp;
extern crate mio;
extern crate ring;
extern crate failure;
extern crate failure_derive;
extern crate slab;

#[macro_use]
extern crate log;
extern crate env_logger;

use failure::Fallible;

use std::env;

mod event_listener;
mod event_stream;

pub mod protocol_capnp {
    include!("../../capnp/protocol_capnp.rs");
}

fn main() -> Fallible<()> {
    env_logger::init();

    let listen_addr = env::var("LISTEN_ADDR").expect("$LISTEN_ADDR must be defined");
    let listen_port = env::var("LISTEN_PORT").expect("$LISTEN_PORT must be defined");
    let addr = format!("{}:{}", listen_addr, listen_port);

    let _listener = event_listener::EventListener::new(&addr)?;
    Ok(())
}

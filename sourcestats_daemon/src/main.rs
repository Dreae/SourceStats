extern crate mio;
extern crate failure;
extern crate failure_derive;
extern crate slab;
extern crate byteorder;
extern crate sourcestats_protocol;
extern crate num_cpus;
extern crate rayon;
extern crate dotenv;

#[macro_use]
extern crate log;
extern crate env_logger;

use failure::Fallible;

use std::env;

mod event_listener;
mod event_stream;
mod packet;
mod db_worker_pool;

use db_worker_pool::DbWorkerService;

fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let listen_addr = env::var("LISTEN_ADDR").expect("$LISTEN_ADDR must be defined");
    let listen_port = env::var("LISTEN_PORT").expect("$LISTEN_PORT must be defined");
    let addr = format!("{}:{}", listen_addr, listen_port);
    let work_pool = DbWorkerService::new(num_cpus::get() * 2);

    let mut listener = event_listener::EventListener::new(&addr, work_pool)?;

    listener.listen()
}

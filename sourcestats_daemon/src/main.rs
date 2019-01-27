extern crate mio;
extern crate failure;
extern crate failure_derive;
extern crate slab;
extern crate byteorder;
extern crate sourcestats_protocol;
extern crate sourcestats_database;
extern crate num_cpus;
extern crate rayon;
extern crate dotenv;
extern crate chrono;

#[macro_use]
extern crate log;
extern crate env_logger;

use failure::Fallible;
use sourcestats_database::Pool;

use std::env;

mod event_listener;
mod event_stream;
mod packet;
mod db_worker_pool;

use db_worker_pool::DbWorkerService;

fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let listen_addr = env::var("LISTEN_ADDR").expect("$LISTEN_ADDR must be set");
    let listen_port = env::var("LISTEN_PORT").expect("$LISTEN_PORT must be set");
    let database_url = env::var("DATABASE_URL").expect("$DATABASE_URL must be set");
    let addr = format!("{}:{}", listen_addr, listen_port);
    let db_pool = Pool::new(num_cpus::get() * 2, &database_url)?;
    let work_pool = DbWorkerService::new(num_cpus::get() * 2, db_pool);

    let mut listener = event_listener::EventListener::new(&addr, work_pool)?;

    listener.listen()
}

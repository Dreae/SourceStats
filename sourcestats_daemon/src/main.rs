extern crate capnp;

use capnp::traits::FromStructBuilder;

pub mod protocol_capnp {
    include!("../../capnp/protocol_capnp.rs");
}

fn main() {
    println!("Hello, world!");
}

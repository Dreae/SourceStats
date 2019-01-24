extern crate capnp;
extern crate ring;
extern crate bytes;
extern crate byteorder;

mod message;

pub mod protocol_capnp {
    include!("../../capnp/protocol_capnp.rs");
}

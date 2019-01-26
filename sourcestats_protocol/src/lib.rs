extern crate capnp;
extern crate ring;
extern crate bytes;
extern crate byteorder;

pub mod message;
pub mod player_update;

pub mod protocol_capnp {
    include!("../../capnp/protocol_capnp.rs");
}

pub use message::*;
pub use player_update::*;
pub use capnp::Error as CapnpError;

use bytes::BytesMut;

trait CapnpSerialize {
    fn serialize(self) -> Result<BytesMut, CapnpError>;
}
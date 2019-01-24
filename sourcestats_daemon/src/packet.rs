use bytes::{Bytes, BytesMut};
use byteorder::{ByteOrder, NetworkEndian};
use mio::Token;

pub struct Packet {
    key: u64,
    nonce: BytesMut,
    data: BytesMut,
    token: Token,
}

impl Packet {
    pub fn from_buf(token: Token, mut buf: BytesMut) -> Packet {
        Packet {
            key: NetworkEndian::read_u64(&buf.split_to(8)),
            nonce: buf.split_to(16),
            data: buf.take(),
            token,
        }
    }
}
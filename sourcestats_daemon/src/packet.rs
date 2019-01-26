use bytes::BytesMut;
use byteorder::{ByteOrder, NetworkEndian};
use mio::Token;

pub struct Packet {
    pub key_id: i64,
    pub nonce: BytesMut,
    pub data: BytesMut,
    pub token: Token,
}

impl Packet {
    pub fn from_buf(token: Token, mut buf: BytesMut) -> Packet {
        Packet {
            key_id: NetworkEndian::read_i64(&buf.split_to(8)),
            nonce: buf.split_to(16),
            data: buf.take(),
            token,
        }
    }

    pub fn serialize(self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(8 + 16 + self.data.len());
        let mut len = [0u8; 8];
        NetworkEndian::write_i64(&mut len, self.key_id);

        buf.extend_from_slice(&len);
        buf.unsplit(self.nonce);
        buf.unsplit(self.data);

        buf
    }
}
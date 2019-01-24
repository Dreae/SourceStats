use capnp::message::{ReaderOptions, Builder, Reader};
use capnp::serialize::OwnedSegments;
use capnp::serialize_packed;
use capnp::Error as CapnpError;
use bytes::{BytesMut, IntoBuf};
use crate::protocol_capnp::player_update;
use ring::aead::{self, OpeningKey, Nonce, CHACHA20_POLY1305, Aad};
use ring::error::Unspecified;
use byteorder::{ByteOrder, NetworkEndian};

#[derive(PartialEq, PartialOrd)]
pub enum MessageType {
    UNKNOWN = 0,
    PlayerUpdate = 1,
}

pub enum Message {
    PlayerUpdate(Reader<OwnedSegments>),
}

pub enum DecryptError {
    CryptographyError,
    InvalidMessage,
    DeserializeError(CapnpError),
}

impl From<Unspecified> for DecryptError {
    fn from(_: Unspecified) -> Self {
        DecryptError::CryptographyError
    }
}

impl From<CapnpError> for DecryptError {
    fn from(e: CapnpError) -> Self {
        DecryptError::DeserializeError(e)
    }
}

impl From<i32> for MessageType {
    fn from(i: i32) -> Self {
        match i {
            1 => MessageType::PlayerUpdate,
            _ => MessageType::UNKNOWN,
        }
    }
}

impl Message {
    pub fn decrypt(key: &[u8], nonce: BytesMut, mut data: BytesMut) -> Result<Message, DecryptError> {
        let key = OpeningKey::new(&CHACHA20_POLY1305, key)?;
        let nonce = Nonce::try_assume_unique_for_key(&nonce[..12])?;
        let plaintext_len = aead::open_in_place(&key, nonce, Aad::empty(), 0, &mut data)?.len();
        let mut buf = data.split_to(plaintext_len);

        let message_id: MessageType = NetworkEndian::read_i32(&buf).into();
        if message_id == MessageType::UNKNOWN {
            return Err(DecryptError::InvalidMessage);
        }
        buf.advance(4);

        match message_id {
            MessageType::PlayerUpdate => {
                let reader = serialize_packed::read_message(&mut buf.into_buf(), ReaderOptions::default())?;
                Ok(Message::PlayerUpdate(reader))
            },
            _ => unreachable!()
        }
    }
}
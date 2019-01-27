use capnp::message::ReaderOptions;
use capnp::serialize_packed;
use capnp::Error as CapnpError;
use bytes::{BytesMut, IntoBuf};
use ring::aead::{self, OpeningKey, Nonce, CHACHA20_POLY1305, Aad, SealingKey};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use byteorder::{ByteOrder, NetworkEndian};
use crate::{PlayerUpdate, CapnpSerialize};

thread_local! {
    pub static CSPRNG: SystemRandom = SystemRandom::new();
}

#[derive(PartialEq, PartialOrd)]
pub enum MessageType {
    UNKNOWN = 0,
    PlayerUpdate = 1,
}

pub enum Message {
    PlayerUpdate(PlayerUpdate),
}

#[derive(Debug)]
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
        buf.advance(4);

        match message_id {
            MessageType::PlayerUpdate => {
                let reader = serialize_packed::read_message(&mut buf.into_buf(), ReaderOptions::default())?;
                Ok(Message::PlayerUpdate(PlayerUpdate::from_capnp(reader)?))
            },
            _ => Err(DecryptError::InvalidMessage)
        }
    }

    pub fn encrypt(key: &[u8], message: Message) -> Result<([u8; 12], BytesMut), DecryptError> {
        let key = SealingKey::new(&CHACHA20_POLY1305, key)?;

        let mut nonce_buf = [0u8; 12];
        CSPRNG.with(|csprng| {
            csprng.fill(&mut nonce_buf)
        })?;

        let nonce = Nonce::assume_unique_for_key(nonce_buf);

        let mut buf = match message {
            Message::PlayerUpdate(update) => update.serialize()?
        };

        buf.reserve(key.algorithm().tag_len());
        unsafe {
            buf.set_len(buf.len() + key.algorithm().tag_len())
        }

        let out_len = aead::seal_in_place(&key, nonce, Aad::empty(), &mut buf, key.algorithm().tag_len())?;
        unsafe {
            buf.set_len(out_len);
        }

        Ok((nonce_buf, buf))
    }
}
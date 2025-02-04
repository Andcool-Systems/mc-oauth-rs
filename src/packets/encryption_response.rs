use anyhow::Result;
use bytes::BytesMut;

use crate::byte_buf_utils::read_varint;

#[allow(dead_code)]
pub struct EncryptionResponsePacket {
    pub shared_secret_length: usize,
    pub shared_secret: Vec<u8>,

    pub verify_token_length: usize,
    pub verify_token: Vec<u8>,
}

impl EncryptionResponsePacket {
    pub fn parse(buff: &mut BytesMut) -> Result<Self> {
        let shared_secret_length = read_varint(buff)?;
        let shared_secret = buff.split_to(shared_secret_length);

        let verify_token_length = read_varint(buff)?;
        let verify_token = buff.split_to(verify_token_length);

        Ok(Self {
            shared_secret_length,
            shared_secret: shared_secret.to_vec(),
            verify_token_length,
            verify_token: verify_token.to_vec(),
        })
    }
}

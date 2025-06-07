use crate::{byte_buf_utils::read_varint, session::Session};
use anyhow::Result;
use bytes::BytesMut;

pub struct EncryptionResponsePacket {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
    pub has_verify: bool,
}

impl EncryptionResponsePacket {
    pub fn parse(server: &mut Session) -> Result<Self> {
        let shared_secret_length = read_varint(&mut server.buffer)?;
        let shared_secret = server.buffer.split_to(shared_secret_length);

        let mut has_verify = false;
        let mut verify_token = BytesMut::new();
        if !(server.proto_ver >= 759 && server.proto_ver <= 760) {
            let verify_token_length = read_varint(&mut server.buffer)?;
            verify_token = server.buffer.split_to(verify_token_length);
            has_verify = true;
        }

        Ok(Self {
            shared_secret: shared_secret.to_vec(),
            verify_token: verify_token.to_vec(),
            has_verify,
        })
    }
}

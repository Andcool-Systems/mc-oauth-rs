use anyhow::Result;
use bytes::{BufMut, BytesMut};

use crate::byte_buf_utils::{add_size, write_utf8, write_varint};

pub struct EncryptionRequestPacket {
    pub server_id: String,
    pub public_key_length: i32,
    pub public_key: Vec<u8>,

    pub verify_token_length: i32,
    pub verify_token: Vec<u8>,
    pub should_authenticate: Option<bool>,
}

impl EncryptionRequestPacket {
    pub fn new(
        server_id: String,
        public_key: Vec<u8>,
        verify_token: Vec<u8>,
        should_authenticate: Option<bool>,
    ) -> Self {
        Self {
            server_id,
            public_key_length: public_key.len() as i32,
            public_key,
            verify_token_length: verify_token.len() as i32,
            verify_token,
            should_authenticate,
        }
    }

    /**
    Build packet for sending over network
    */
    pub fn build(&self) -> Result<BytesMut> {
        let mut buffer = BytesMut::new();

        write_varint(&mut buffer, 0x01); // Packet id
        write_utf8(&mut buffer, &self.server_id)?; // Server id

        write_varint(&mut buffer, self.public_key_length);
        buffer.extend_from_slice(&self.public_key);

        write_varint(&mut buffer, self.verify_token_length);
        buffer.extend_from_slice(&self.verify_token);

        if let Some(v) = self.should_authenticate {
            buffer.put_u8(if v { 0x01 } else { 0x00 });
        }

        Ok(add_size(&buffer))
    }
}

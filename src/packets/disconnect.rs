use anyhow::Result;
use bytes::BytesMut;

use crate::byte_buf_utils::{add_size, write_utf8, write_varint};

pub struct DisconnectPacket {
    pub reason: String,
}

impl DisconnectPacket {
    pub fn build(&self) -> Result<BytesMut> {
        let mut buffer = BytesMut::new();

        write_varint(&mut buffer, 0x00); // Packet id
        write_utf8(&mut buffer, &self.reason)?; // Server id

        Ok(add_size(&buffer))
    }
}

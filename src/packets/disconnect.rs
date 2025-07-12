use anyhow::Result;
use bytes::BytesMut;

use crate::byte_buf_utils::{add_size, write_utf8, write_varint};

pub struct DisconnectPacket {
    pub reason: String,
}

impl Default for DisconnectPacket {
    /// Create empty disconnect packet
    fn default() -> Self {
        Self {
            reason: String::new(),
        }
    }
}

impl DisconnectPacket {
    /// Create disconnect packet with provided reason
    pub fn with_reason(reason: String) -> Self {
        Self { reason }
    }

    /// Build packet for sending over network
    pub fn build(&self) -> Result<BytesMut> {
        let mut buffer = BytesMut::new();

        write_varint(&mut buffer, 0x00); // Packet id
        write_utf8(&mut buffer, &format!(r#"{{"text":"{}"}}"#, self.reason))?; // Disconnect reason

        Ok(add_size(&buffer))
    }
}

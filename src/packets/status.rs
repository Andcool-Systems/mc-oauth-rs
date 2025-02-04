use anyhow::Result;
use bytes::BytesMut;
use serde_json::{json, Value};

use crate::byte_buf_utils::{add_size, write_utf8, write_varint};

pub struct StatusPacket {}

impl StatusPacket {
    pub fn build(data: StatusData) -> Result<BytesMut> {
        let mut buffer = BytesMut::new();

        write_varint(&mut buffer, 0x00); // Packet id
        write_utf8(&mut buffer, &data.build().to_string())?;

        Ok(add_size(&buffer))
    }
}

pub struct StatusData {
    pub version_name: String,
    pub version_protocol: usize,
    pub players_max: usize,
    pub players_online: usize,
    pub description: Value,
    pub favicon: String,
    pub enforces_secure_chat: bool,
}

impl StatusData {
    pub fn build(&self) -> Value {
        json!({
            "version": {
                "name": self.version_name,
                "protocol": self.version_protocol
            },
            "players": {
                "max": self.players_max,
                "online": self.players_online,
                "sample": []
            },
            "description": self.description,
            "favicon": self.favicon,
            "enforcesSecureChat": self.enforces_secure_chat
        })
    }
}

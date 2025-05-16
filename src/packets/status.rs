use crate::byte_buf_utils::{add_size, write_utf8, write_varint};
use anyhow::Result;
use bytes::BytesMut;
use serde::Serialize;
use serde_json::Value;

pub struct StatusPacket {}

impl StatusPacket {
    pub fn build(data: StatusData) -> Result<BytesMut> {
        let mut buffer = BytesMut::new();

        write_varint(&mut buffer, 0x00); // Packet id
        write_utf8(&mut buffer, &serde_json::to_string(&data)?)?;

        Ok(add_size(&buffer))
    }
}

#[derive(Serialize)]
pub struct StatusData {
    pub version: Version,
    pub players: Players,
    pub description: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    pub enforces_secure_chat: bool,
}

#[derive(Serialize)]
pub struct Players {
    pub max: usize,
    pub online: usize,
    pub sample: Vec<Value>,
}

#[derive(Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: usize,
}

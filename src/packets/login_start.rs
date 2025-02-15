use bytes::BytesMut;
use uuid::Uuid;

use crate::byte_buf_utils::{read_utf8, try_get_uuid};

#[allow(dead_code)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Option<Uuid>,
}

impl LoginStartPacket {
    pub fn parse(buff: &mut BytesMut) -> anyhow::Result<Self> {
        Ok(Self {
            name: read_utf8(buff)?,
            uuid: try_get_uuid(buff).ok()
        })
    }
}

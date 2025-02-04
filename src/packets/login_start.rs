use bytes::BytesMut;
use tokio::io;

use crate::byte_buf_utils::read_utf8;

#[allow(dead_code)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: String,
}

impl LoginStartPacket {
    pub fn parse(buff: &mut BytesMut) -> Result<Self, io::Error> {
        Ok(Self {
            name: read_utf8(buff)?,
            uuid: "".to_string(),
        })
    }
}

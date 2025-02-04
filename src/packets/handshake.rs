use bytes::BytesMut;
use tokio::io;

use crate::byte_buf_utils::{read_unsigned_short, read_utf8, read_varint};

#[allow(dead_code)]
pub struct HandshakePacket {
    pub proto_ver: usize,
    pub server_addr: String,
    pub port: u16,
    pub next_state: usize,
}

impl HandshakePacket {
    pub fn parse(buff: &mut BytesMut) -> Result<Self, io::Error> {
        Ok(Self {
            proto_ver: read_varint(buff)?,
            server_addr: read_utf8(buff)?,
            port: read_unsigned_short(buff)?,
            next_state: read_varint(buff)?
        })
    }
}

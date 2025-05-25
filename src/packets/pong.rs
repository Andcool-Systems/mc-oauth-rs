use crate::byte_buf_utils::{add_size, write_varint};
use anyhow::Result;
use bytes::{BufMut, BytesMut};

pub struct PongPacket {
    pub payload: i64,
}

impl PongPacket {
    /**
    Build packet for sending over network
    */
    pub fn build(&self) -> Result<BytesMut> {
        let mut buffer = BytesMut::new();

        write_varint(&mut buffer, 0x01); // Packet id
        buffer.put_i64(self.payload);

        Ok(add_size(&buffer))
    }
}

use bytes::{Buf, BufMut, BytesMut};
use std::io::{self, ErrorKind};

pub fn read_utf8(buf: &mut BytesMut) -> io::Result<String> {
    let len = read_varint(buf)?;
    if buf.len() < len {
        return Err(io::Error::new(ErrorKind::UnexpectedEof, "Not enough data"));
    }

    let bytes = buf.split_to(len);
    match String::from_utf8(bytes.to_vec()) {
        Ok(s) => Ok(s),
        Err(_) => Err(io::Error::new(ErrorKind::InvalidData, "UTF-8 decode error")),
    }
}

pub fn write_utf8(buf: &mut BytesMut, value: &str) -> io::Result<()> {
    let bytes = value.as_bytes();
    if bytes.len() >= 32767 {
        return Err(io::Error::new(ErrorKind::InvalidInput, "String too long"));
    }

    write_varint(buf, bytes.len() as i32);
    buf.extend_from_slice(bytes);
    Ok(())
}

pub fn read_varint(buf: &mut BytesMut) -> io::Result<usize> {
    let mut value = 0;
    let mut bytes = 0;

    while let Some(byte) = buf.first().copied() {
        buf.advance(1);
        value |= ((byte & 0x7F) as usize) << (bytes * 7);
        bytes += 1;

        if bytes > 5 {
            return Err(io::Error::new(ErrorKind::InvalidData, "VarInt too big"));
        }
        if byte & 0x80 == 0 {
            break;
        }
    }

    Ok(value)
}

pub fn write_varint(buf: &mut BytesMut, value: i32) {
    let mut val = value as u32;
    loop {
        let mut temp = (val & 0x7F) as u8;
        val >>= 7;
        if val != 0 {
            temp |= 0x80;
        }
        buf.put_u8(temp);
        if val == 0 {
            break;
        }
    }
}

pub fn add_size(data: &BytesMut) -> BytesMut {
    let mut packet = BytesMut::new();
    write_varint(&mut packet, data.len() as i32);
    packet.extend_from_slice(&data);
    packet
}

pub fn read_unsigned_short(buf: &mut BytesMut) -> io::Result<u16> {
    if buf.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Not enough data",
        ));
    }
    Ok(buf.get_u16())
}

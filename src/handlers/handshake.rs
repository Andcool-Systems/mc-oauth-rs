use std::io;

use crate::{client_sessions::Session, packets::handshake::HandshakePacket};
use bytes::BytesMut;

pub fn handle(session: &mut Session, buff: &mut BytesMut) -> Result<(), io::Error> {
    let handshake = HandshakePacket::parse(buff)?;

    session.proto_ver = Some(handshake.proto_ver);
    session.next_state = Some(handshake.next_state);

    Ok(())
}

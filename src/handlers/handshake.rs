use crate::{
    client_sessions::{NextStateEnum, Session},
    packets::handshake::HandshakePacket,
};
use bytes::BytesMut;
use std::io;

pub fn handle(session: &mut Session, buff: &mut BytesMut) -> Result<(), io::Error> {
    let handshake = HandshakePacket::parse(buff)?;

    session.next_state = match handshake.next_state {
        1 => NextStateEnum::Status,
        2 => NextStateEnum::Login,
        _ => NextStateEnum::Unknown,
    };

    session.proto_ver = Some(handshake.proto_ver);

    Ok(())
}

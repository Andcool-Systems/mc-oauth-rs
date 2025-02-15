use crate::{client_sessions::Session, packets::login_start::LoginStartPacket};
use bytes::BytesMut;

pub fn handle(session: &mut Session, buff: &mut BytesMut) -> anyhow::Result<()> {
    let packet = LoginStartPacket::parse(buff)?;
    session.nickname = Some(packet.name);
    session.uuid = packet.uuid;
    Ok(())
}

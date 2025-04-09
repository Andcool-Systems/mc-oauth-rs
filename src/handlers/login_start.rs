use crate::{client::Session, packets::login_start::LoginStartPacket};
use bytes::BytesMut;

pub fn handle_login_start(session: &mut Session, buff: &mut BytesMut) -> anyhow::Result<()> {
    let packet = LoginStartPacket::parse(buff)?;
    session.nickname = Some(packet.name);
    session.uuid = packet.uuid;
    Ok(())
}

use crate::{client_sessions::Session, packets::login_start::LoginStartPacket};
use bytes::BytesMut;
use tokio::io;

pub fn handle(session: &mut Session, buff: &mut BytesMut) -> Result<(), io::Error> {
    let packet = LoginStartPacket::parse(buff)?;
    println!("Login start for {:}", packet.name);

    session.nickname = Some(packet.name);
    Ok(())
}

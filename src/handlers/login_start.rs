use crate::{packets::login_start::LoginStartPacket, server::MinecraftServer};

impl MinecraftServer {
    /**
    Handle login start packet from client
    */
    pub fn handle_login_start(&mut self) -> anyhow::Result<()> {
        let packet = LoginStartPacket::parse(&mut self.buffer)?;
        self.session.nickname = Some(packet.name);
        self.session.uuid = packet.uuid;
        Ok(())
    }
}

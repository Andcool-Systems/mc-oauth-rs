use crate::{packets::login_start::LoginStartPacket, server::MinecraftServer};
use anyhow::Error;

impl MinecraftServer {
    /**
    Handle login start packet from client
    */
    pub async fn handle_login_start(&mut self) -> anyhow::Result<()> {
        let packet = LoginStartPacket::parse(&mut self.buffer)?;
        self.session.nickname = Some(packet.name);
        self.session.uuid = packet.uuid;

        if self.session.proto_ver >= 759 && self.session.proto_ver <= 762 {
            self.send_disconnect(
                "Sorry, Minecraft 1.19.* clients not supported yet.\nTry using another version of client."
                    .to_string(),
            )
            .await?;
            return Err(Error::msg("Client using unsupported version!"));
        }
        Ok(())
    }
}

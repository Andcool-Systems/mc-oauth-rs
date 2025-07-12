use crate::{packets::login_start::LoginStartPacket, session::Session};
use anyhow::Error;

impl Session {
    /// Handle login start packet from client
    pub async fn handle_login_start(&mut self) -> anyhow::Result<()> {
        let packet = LoginStartPacket::parse(&mut self.buffer)?;
        self.nickname = Some(packet.name);
        self.uuid = packet.uuid;

        if self.proto_ver >= 759 && self.proto_ver <= 760 && !self.config.server.support_1_19 {
            self.send_disconnect(self.config.messages.unsupported_client_version.clone())
                .await?;
            return Err(Error::msg("Client using unsupported version (1.19)!"));
        }
        Ok(())
    }
}

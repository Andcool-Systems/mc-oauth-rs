use anyhow::{Error, Result};

use crate::{
    packets::handshake::HandshakePacket,
    server::{MinecraftServer, NextStateEnum},
};

impl MinecraftServer {
    /**
    Handle handshake packet

    Also check client's connection host
    */
    pub async fn handle_handshake(&mut self) -> Result<()> {
        let handshake = HandshakePacket::parse(&mut self.buffer)?;

        self.session.proto_ver = handshake.proto_ver;
        self.session.next_state = match handshake.next_state {
            1 => NextStateEnum::Status,
            2 => NextStateEnum::Login,
            _ => NextStateEnum::Unknown,
        };

        // Check client's connecting hostname
        if let Some(server_ip) = &self.config.server.server_ip {
            if server_ip.ne(&handshake.server_addr) {
                self.send_disconnect(self.config.messages.using_proxy.clone())
                    .await?;
                return Err(Error::msg("Client using a proxy!"));
            }
        }

        Ok(())
    }
}

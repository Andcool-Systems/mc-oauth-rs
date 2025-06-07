use anyhow::{Error, Result};

use crate::{
    packets::handshake::HandshakePacket,
    session::{NextStateEnum, Session},
};

impl Session {
    /**
    Handle handshake packet

    Also check client's connection host
    */
    pub async fn handle_handshake(&mut self) -> Result<()> {
        let handshake = HandshakePacket::parse(&mut self.buffer)?;

        self.proto_ver = handshake.proto_ver;
        self.next_state = match handshake.next_state {
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

        // Check client's protocol version
        let server_proto_ver = self.config.server.config.protocol;
        if let NextStateEnum::Login = self.next_state {
            if server_proto_ver.ne(&0) && self.proto_ver.ne(&server_proto_ver) {
                self.send_disconnect(self.config.messages.unsupported_client_version.clone())
                    .await?;
                return Err(Error::msg(format!(
                    "Unsupported client version! Server: {}, client: {}",
                    server_proto_ver, self.proto_ver
                )));
            }
        }

        Ok(())
    }
}

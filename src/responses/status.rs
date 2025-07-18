use crate::{
    config::get_config,
    packets::status::{self, StatusData, StatusPacket},
    session::Session,
};
use anyhow::Result;
use serde_json::json;
use tokio::io::AsyncWriteExt;

impl Session {
    /// Send status response
    pub async fn send_status(&mut self) -> Result<()> {
        let config = get_config().await;
        let proto_ver = if config.server.config.protocol == 0 {
            self.proto_ver
        } else {
            config.server.config.protocol
        };

        let packet = StatusPacket::new(StatusData {
            version: status::Version {
                name: config.server.config.version.clone(),
                protocol: proto_ver,
            },
            players: status::Players {
                max: config.server.status.players_max,
                online: config.server.status.players_online,
                sample: vec![],
            },
            description: json!({"text": config.server.status.description.clone()}),
            favicon: config.image.clone(),
            enforces_secure_chat: false,
        });

        self.stream.writable().await?;
        self.stream.write_all(&packet.build()?).await?;
        Ok(())
    }
}

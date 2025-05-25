use anyhow::Result;

use crate::{config::get_config, packets::status, server::MinecraftServer};
use serde_json::json;
use tokio::io::AsyncWriteExt;

impl MinecraftServer {
    /**
    Send status response
    */
    pub async fn send_status(&mut self) -> Result<()> {
        let config = get_config().await;
        let proto_ver = if config.server.config.protocol == 0 {
            match self.session.proto_ver {
                Some(x) => x,
                None => unreachable!(),
            }
        } else {
            config.server.config.protocol
        };

        let data = status::StatusData {
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
        };

        self.stream.writable().await?;
        self.stream
            .write_all(&status::StatusPacket::build(data)?)
            .await?;
        Ok(())
    }
}

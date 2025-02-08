use anyhow::Result;

use serde_json::json;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{client_sessions::Session, config::get_config, packets::status};

pub async fn send(stream: &mut TcpStream, session: &mut Session) -> Result<()> {
    let config = get_config().await;
    let proto_ver = if config.server.config.protocol == 0 {
        session.proto_ver.unwrap()
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

    stream.writable().await?;
    stream
        .write_all(&status::StatusPacket::build(data)?)
        .await?;
    Ok(())
}

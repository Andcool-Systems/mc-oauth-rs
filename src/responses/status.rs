use anyhow::Result;

use serde_json::json;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{
    client_sessions::Session,
    config::get_config,
    packets::status::{StatusData, StatusPacket},
};

pub async fn send(stream: &mut TcpStream, session: &mut Session) -> Result<()> {
    let config = get_config().await;
    let proto_ver = if config.proto_ver == 0 {
        session.proto_ver.unwrap()
    } else {
        config.proto_ver
    };

    let icon = match &config.image {
        Some(img) => img,
        None => &"".to_string()
    };

    let data = StatusData {
        version_name: config.server_ver.clone(),
        version_protocol: proto_ver,
        players_max: config.players_max,
        players_online: config.players_online,
        description: json!({"text": config.motd.clone()}),
        favicon: icon.to_string(),
        enforces_secure_chat: false,
    };

    stream.write_all(&StatusPacket::build(data)?).await?;
    Ok(())
}

use anyhow::Result;

use serde_json::json;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{
    client_sessions::Session,
    packets::status::{StatusData, StatusPacket},
};

pub async fn send(stream: &mut TcpStream, session: &mut Session) -> Result<()> {
    let data = StatusData {
        version_name: "1.21.4".to_string(),
        version_protocol: session.proto_ver.unwrap(),
        players_max: 0,
        players_online: 0,
        description: json!({"text": "Hello, world!"}),
        favicon: "".to_string(),
        enforces_secure_chat: false,
    };

    stream.write_all(&StatusPacket::build(data)?).await?;
    Ok(())
}

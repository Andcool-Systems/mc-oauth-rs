use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::debug;

use crate::{
    client_sessions::Session, encryption::encrypt_packet, packets::disconnect::DisconnectPacket,
};

pub async fn send(stream: &mut TcpStream, session: &mut Session, reason: String) -> Result<()> {
    let mut disconnect_packet = DisconnectPacket {
        reason: format!(r#"{{"text":"{}"}}"#, reason),
    }
    .build()?;

    encrypt_packet(&mut disconnect_packet, session);

    stream.writable().await?;
    stream.write_all(&disconnect_packet).await?;
    stream.shutdown().await?;

    debug!("Disconnected client with reason: {}", reason);
    Ok(())
}

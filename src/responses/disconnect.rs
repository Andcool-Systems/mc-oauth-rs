use crate::{client::Session, encryption::encrypt_packet, packets::disconnect::DisconnectPacket};
use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::debug;

pub async fn send_disconnect(
    stream: &mut TcpStream,
    session: &mut Session,
    reason: String,
) -> Result<()> {
    let mut disconnect_packet = DisconnectPacket {
        reason: format!(r#"{{"text":"{}"}}"#, reason),
    }
    .build()?;

    // Passing a session object allows sending a disconnect packet with or without encryption.
    encrypt_packet(&mut disconnect_packet, session);

    stream.writable().await?;
    stream.write_all(&disconnect_packet).await?;

    // Minecraft clients expect that after sending a disconnect packet there should be a disconnect
    stream.shutdown().await?;

    debug!("Disconnected client with reason: {}", reason);
    Ok(())
}

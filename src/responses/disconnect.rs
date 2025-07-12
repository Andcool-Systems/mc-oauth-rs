use crate::{packets::disconnect::DisconnectPacket, session::Session};
use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tracing::debug;

impl Session {
    /// Send disconnect packet with provided reason
    pub async fn send_disconnect(&mut self, reason: String) -> Result<()> {
        let mut disconnect_packet = DisconnectPacket::with_reason(reason.clone()).build()?;

        // Passing a session object allows sending a disconnect packet with or without encryption.
        self.encrypt_packet(&mut disconnect_packet);

        self.stream.writable().await?;
        self.stream.write_all(&disconnect_packet).await?;

        // Minecraft clients expect that after sending a disconnect packet there should be a disconnect
        self.stream.shutdown().await?;

        debug!("Disconnected client with reason: {}", reason);
        Ok(())
    }
}

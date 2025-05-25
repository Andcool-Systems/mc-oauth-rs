use anyhow::Result;

use crate::{packets::ping::PingPacket, server::MinecraftServer};
use tokio::io::AsyncWriteExt;

impl MinecraftServer {
    /**
    Send ping response
    */
    pub async fn send_ping(&mut self, payload: i64) -> Result<()> {
        let packet = PingPacket { payload };

        self.stream.writable().await?;
        self.stream.write_all(&packet.build()?).await?;
        Ok(())
    }
}

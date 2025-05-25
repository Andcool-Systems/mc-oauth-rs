use anyhow::Result;

use crate::{packets::pong::PongPacket, server::MinecraftServer};
use tokio::io::AsyncWriteExt;

impl MinecraftServer {
    /**
    Send pong response
    */
    pub async fn send_pong(&mut self, payload: i64) -> Result<()> {
        let packet = PongPacket { payload };

        self.stream.writable().await?;
        self.stream.write_all(&packet.build()?).await?;
        Ok(())
    }
}

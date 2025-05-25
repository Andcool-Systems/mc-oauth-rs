use anyhow::Result;

use crate::{packets::ping::PingPacket, server::MinecraftServer};
use bytes::Buf;
use tokio::io::AsyncWriteExt;

impl MinecraftServer {
    /**
    Handle ping and send response
    */
    pub async fn handle_ping(&mut self) -> Result<()> {
        let payload = self.buffer.get_i64();
        let packet = PingPacket { payload };

        self.stream.writable().await?;
        self.stream.write_all(&packet.build()?).await?;
        Ok(())
    }
}

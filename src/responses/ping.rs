use anyhow::Result;

use crate::packets::ping::PingPacket;
use bytes::{Buf, BytesMut};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn handle_ping(stream: &mut TcpStream, buff: &mut BytesMut) -> Result<()> {
    let payload = buff.get_i64();
    let packet = PingPacket { payload };

    stream.writable().await?;
    stream.write_all(&packet.build()?).await?;
    Ok(())
}

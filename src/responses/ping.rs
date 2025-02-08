use anyhow::Result;

use bytes::{Buf, BytesMut};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::packets::ping::PingPacket;

pub async fn handle_and_send(stream: &mut TcpStream, buff: &mut BytesMut) -> Result<()> {
    let payload = buff.get_i64();
    let packet = PingPacket { payload };

    stream.writable().await?;
    stream.write_all(&packet.build()?).await?;
    Ok(())
}

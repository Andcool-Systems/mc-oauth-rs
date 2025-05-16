use anyhow::{Error, Result};

use crate::{
    client::{MinecraftClient, NextStateEnum},
    packets::handshake::HandshakePacket,
    responses::disconnect::send_disconnect,
};

pub async fn handle_handshake(client: &mut MinecraftClient) -> Result<()> {
    let handshake = HandshakePacket::parse(&mut client.buffer)?;

    client.session.next_state = match handshake.next_state {
        1 => NextStateEnum::Status,
        2 => NextStateEnum::Login,
        _ => NextStateEnum::Unknown,
    };

    // Check client's connecting hostname
    client.session.proto_ver = Some(handshake.proto_ver);
    if let Some(server_ip) = &client.config.server.server_ip {
        if server_ip.ne(&handshake.server_addr) {
            send_disconnect(
                &mut client.stream,
                &mut client.session,
                client.config.messages.using_proxy.clone(),
            )
            .await?;
            return Err(Error::msg("Client using a proxy!"));
        }
    }

    Ok(())
}

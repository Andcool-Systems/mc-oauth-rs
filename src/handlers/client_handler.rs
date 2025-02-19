use std::{sync::Arc, time::Duration};

use anyhow::Result;
use bytes::{BufMut, BytesMut};
use tokio::net::TcpStream;
use tracing::{debug, info};

use crate::{
    byte_buf_utils::read_varint,
    client_sessions::{NextStateEnum, Session},
    config::get_config,
    generators::keys::generate_code,
    handlers::{encryption_response, handshake, login_start},
    map::get_map,
    mojang,
    responses::{disconnect, encryption, ping, status},
};

pub async fn handle(mut stream: TcpStream, keys: Arc<rsa::RsaPrivateKey>) -> Result<()> {
    let mut buffer = BytesMut::new();
    let session = &mut Session::new().await; // Create session for current client
    let config = get_config().await;

    loop {
        let mut temp_buf = vec![0; 1024];
        stream.readable().await?;

        match stream.try_read(&mut temp_buf) {
            Ok(0) => {
                debug!("No more data to read. Exiting...");
                break; // if client disconnected
            }
            Ok(n) => {
                buffer.put_slice(&temp_buf[..n]);

                while packet_available(&mut buffer) {
                    let packet_id = read_varint(&mut buffer)?;
                    debug!("Received packet: {}", packet_id);

                    match packet_id {
                        0x00 => match session.next_state {
                            NextStateEnum::Status => status::send(&mut stream, session).await?, // Send status response
                            NextStateEnum::Login => {
                                // Handle login start
                                login_start::handle(session, &mut buffer)?;
                                encryption::send(&mut stream, keys.clone(), session).await?;
                            }
                            NextStateEnum::Unknown => handshake::handle(session, &mut buffer)?, // Handle handshake
                        },

                        0x01 => match session.next_state {
                            NextStateEnum::Status => {
                                // Handle ping request
                                ping::handle_and_send(&mut stream, &mut buffer).await?
                            }
                            NextStateEnum::Login => {
                                debug!("Received encryption response");
                                encryption_response::handle(session, &mut buffer, keys.clone())
                                    .expect("Encryption response handling error");

                                let player_data = mojang::join(session, keys.clone()).await?;
                                if player_data.is_none() {
                                    debug!("Mojang error");
                                    disconnect::send(
                                        &mut stream,
                                        session,
                                        config.messages.bad_session.clone(),
                                    )
                                    .await?;
                                    return Ok(());
                                }
                                let player_data = player_data.unwrap();
                                let map = get_map().await;
                                let code = generate_code(6); // Generate 6-digit code

                                // Insert client data into hash map
                                map.insert(
                                    code.clone(),
                                    player_data.clone(),
                                    Duration::from_secs(config.api.code_life_time),
                                )
                                .await;

                                // Disconnect client with code
                                disconnect::send(
                                    &mut stream,
                                    session,
                                    config
                                        .messages
                                        .success
                                        .replace("{{NAME}}", &player_data.name)
                                        .replace("{{UUID}}", &player_data.id)
                                        .replace("{{CODE}}", &code),
                                )
                                .await?;

                                info!("Created code {} for {}", code, player_data.name);
                                return Ok(());
                            }
                            NextStateEnum::Unknown => return Ok(()),
                        },
                        _ => return Ok(()),
                    }
                }
            }
            Err(_) => {}
        }
        buffer.clear();
    }
    Ok(())
}

fn packet_available(buffer: &mut BytesMut) -> bool {
    if buffer.is_empty() {
        return false;
    }
    // Read packet length
    read_varint(buffer)
        .map(|x| buffer.len() >= x)
        .unwrap_or(false)
}

use std::{sync::Arc, time::Duration};

use anyhow::{Error, Result};
use bytes::{BufMut, BytesMut};
use tokio::net::TcpStream;

use crate::{
    byte_buf_utils::read_varint,
    client_sessions::{NextStateEnum, Session},
    generators::keys::generate_code,
    handlers::{encryption_response, handshake, login_start},
    map::{get_map, MinecraftData},
    mojang,
    responses::{disconnect, encryption, ping, status},
};

pub async fn handle(mut stream: TcpStream, keys: Arc<rsa::RsaPrivateKey>) -> Result<()> {
    let mut buffer = BytesMut::new();
    let session = &mut Session::new();

    loop {
        let mut temp_buf = vec![0; 1024];
        stream.readable().await?;

        match stream.try_read(&mut temp_buf) {
            Ok(0) => break,
            Ok(n) => {
                buffer.put_slice(&temp_buf[..n]);

                read_varint(&mut buffer)?; // Packet length
                let packet_id = read_varint(&mut buffer)?;

                match packet_id {
                    0x00 => {
                        handshake::handle(session, &mut buffer)?; // Handle handshake
                        handle_login_start(&mut buffer, session, &mut stream, keys.clone()).await?;
                    }

                    0x01 => match session.next_state {
                        NextStateEnum::Status => {
                            ping::handle_and_send(&mut stream, &mut buffer).await?
                        }
                        NextStateEnum::Login => {
                            encryption_response::handle(session, &mut buffer, keys.clone())?;

                            let player_data = mojang::join(session, keys).await?;
                            if player_data.is_none() {
                                disconnect::send(
                                    &mut stream,
                                    session,
                                    "Failed to login: Invalid session (Try restarting your game and the launcher)".to_string()
                                ).await?;
                                break;
                            }
                            let player_data = player_data.unwrap();

                            let map = get_map().await;
                            let player_struct = MinecraftData {
                                name: player_data.name.clone(),
                                uuid: player_data.id,
                            };

                            let code = generate_code();
                            map.insert(
                                code.clone(),
                                player_struct,
                                Duration::from_secs(60 * 5),
                            )
                            .await;

                            disconnect::send(
                                &mut stream,
                                session,
                                format!("Hello, {}! Your code is: {}", player_data.name, code),
                            )
                            .await?;
                            break;
                        }
                        NextStateEnum::Unknown => break,
                    },
                    _ => break,
                }
            }
            Err(_) => {}
        }
        buffer.clear();
    }

    println!("Disconnected!");
    Ok(())
}

async fn handle_login_start(
    buffer: &mut BytesMut,
    session: &mut Session,
    stream: &mut TcpStream,
    keys: Arc<rsa::RsaPrivateKey>,
) -> Result<()> {
    read_varint(buffer)?;
    read_varint(buffer)?;

    match session.next_state {
        NextStateEnum::Status => status::send(stream, session).await?, // Send status response
        NextStateEnum::Login => {
            login_start::handle(session, buffer)?; // Handle login start
            encryption::send(stream, keys.clone(), session).await?; // Send encryption request
        }
        NextStateEnum::Unknown => {
            println!("Unknown next state");
            return Err(Error::msg("Unknown next state"));
        }
    }

    Ok(())
}

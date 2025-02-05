use std::{sync::Arc, time::Duration};

use anyhow::Result;
use bytes::{BufMut, BytesMut};
use tokio::net::TcpStream;

use crate::{
    byte_buf_utils::read_varint,
    client_sessions::{NextStateEnum, Session},
    generators::keys::generate_code,
    handlers::{encryption_response, handshake, login_start},
    map::get_map,
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

                while packet_available(&mut buffer) {
                    let packet_id = read_varint(&mut buffer)?;

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
                                ping::handle_and_send(&mut stream, &mut buffer).await?
                            }
                            NextStateEnum::Login => {
                                encryption_response::handle(session, &mut buffer, keys.clone())?;

                                let player_data = mojang::join(session, keys.clone()).await?;
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
                                let code = generate_code();

                                map.insert(
                                    code.clone(),
                                    player_data.clone(),
                                    Duration::from_secs(60 * 5),
                                )
                                .await;

                                disconnect::send(
                                    &mut stream,
                                    session,
                                    format!("Hello, {}! Your code is: {}", player_data.name, code),
                                )
                                .await?;

                                println!("Created code {} for {}", code, player_data.name);
                                break;
                            }
                            NextStateEnum::Unknown => break,
                        },
                        _ => break,
                    }
                }
            }
            Err(_) => {}
        }
        buffer.clear();
    }

    println!("Disconnected!");
    Ok(())
}


fn packet_available(buffer: &mut BytesMut) -> bool {
    if buffer.len() == 0 {
        return false;
    }

    // Read packet length
    match read_varint(buffer) {
        Ok(packet_len) => buffer.len() >= packet_len,
        Err(_) => false,
    }
}

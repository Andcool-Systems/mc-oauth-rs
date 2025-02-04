mod byte_buf_utils;
mod client_sessions;
mod encryption;
mod generators;
mod handlers;
mod mojang;
mod packets;
mod responses;

use std::sync::Arc;

use anyhow::Result;
use byte_buf_utils::read_varint;
use bytes::{BufMut, BytesMut};
use client_sessions::Session;
use generators::keys::{generate_key_pair, generate_verify_token};
use responses::{disconnect, ping, status};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut stream: TcpStream, keys: Arc<rsa::RsaPrivateKey>) -> Result<()> {
    let mut buffer = BytesMut::new();
    let verify_token = generate_verify_token();
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
                println!("ID: {:?}", packet_id);

                match packet_id {
                    0x00 => {
                        handlers::handshake::handle(session, &mut buffer)?;

                        read_varint(&mut buffer)?;
                        read_varint(&mut buffer)?;
                        match session.next_state {
                            Some(1) => status::send(&mut stream, session).await?,
                            Some(2) => {
                                handlers::login_start::handle(session, &mut buffer)?;
                                responses::encryption::send(
                                    &mut stream,
                                    keys.clone(),
                                    verify_token,
                                    session,
                                )
                                .await?;
                            }
                            Some(next_state) => {
                                println!("Unknown next state: {}", next_state);
                                break;
                            }
                            None => todo!(),
                        }
                    }

                    0x01 => match session.next_state {
                        Some(1) => ping::handle_and_send(&mut stream, &mut buffer).await?,
                        Some(2) => {
                            handlers::encryption_response::handle(
                                session,
                                &mut buffer,
                                keys.clone(),
                                verify_token,
                            )?;

                            let mojang_api_response = mojang::join(session, keys).await?;
                            match mojang_api_response {
                                Some(r) => {
                                    disconnect::send(
                                        &mut stream,
                                        session,
                                        format!("Hello, {}", r.name),
                                    )
                                    .await?
                                }
                                None => {
                                    disconnect::send(
                                        &mut stream,
                                        session,
                                        "Mojang API error".to_string(),
                                    )
                                    .await?
                                }
                            }
                            break;
                        }
                        _ => {}
                    },
                    _ => break,
                }

                //println!("Получено: {:?}", &buffer[..]);
            }
            Err(_e) => {}
        }
        buffer.clear();
    }

    println!("Disconnected!");
    Ok(())
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:1488").await?;
    let keys = Arc::new(generate_key_pair());
    //let sessions: SessionMap = Arc::new(Mutex::new(HashMap::new()));
    println!("Сервер запущен на 0.0.0.0:1488");

    loop {
        let (stream, addr) = listener.accept().await?;
        let keys = keys.clone();
        println!("Новое соединение от: {}", addr);

        tokio::spawn(async move {
            handle_client(stream, keys).await.expect("Ne ebu");
        });
    }
}

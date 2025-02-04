mod api;
mod byte_buf_utils;
mod client_sessions;
mod encryption;
mod generators;
mod handlers;
mod map;
mod mojang;
mod packets;
mod responses;

use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

use api::build_http_server;
use generators::keys::generate_key_pair;
use handlers::client_handler;
use map::init_map;
use tokio::signal;
use tokio::{net::TcpListener, sync::Notify};

const ADDRESS: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 1488);
const ADDRESS_API: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8008);

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let http = build_http_server(ADDRESS_API);
    tokio::spawn(async move { http.await });

    let listener = TcpListener::bind(ADDRESS).await?;
    let keys = Arc::new(generate_key_pair());
    init_map();

    let shutdown_signal = signal::ctrl_c();
    let shutdown_notify = Arc::new(Notify::new());

    tokio::spawn({
        let shutdown_notify = shutdown_notify.clone();
        async move {
            match shutdown_signal.await {
                Ok(()) => {
                    println!("Shutdown signal received. Closing server...");
                    shutdown_notify.notify_one();
                }
                Err(err) => {
                    eprintln!("Failed to listen for shutdown signal: {}", err);
                }
            }
        }
    });

    loop {
        tokio::select! {
            result = listener.accept() => {
                match result {
                    Ok((stream, addr)) => {
                        let keys = keys.clone();
                        println!("New connection from: {}", addr);
                        tokio::spawn(async move {
                            if let Err(e) = client_handler::handle(stream, keys).await {
                                eprintln!("Error handling client {}: {}", addr, e);
                            }
                        });
                    },
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                    }
                }
            },
            _ = shutdown_notify.notified() => {
                println!("Shutting down the server...");
                break;
            },
        }
    }

    Ok(())
}

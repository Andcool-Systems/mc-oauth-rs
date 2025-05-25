mod api;
mod byte_buf_utils;
mod config;
mod encryption;
mod generators;
mod handlers;
mod map;
mod mojang;
mod packets;
mod responses;
mod server;

use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use api::build_http_server;
use generators::generate_key_pair;
use server::MinecraftServer;
use tokio::{net::TcpListener, sync::Notify};
use tokio::{signal, time::timeout};
use tracing::{error, info};

fn init_logger(level: tracing::Level) {
    use tracing_subscriber::FmtSubscriber;

    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .compact()
            .with_max_level(level)
            .without_time()
            .finish(),
    )
    .expect("Fail to set global default subscriber");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up logger
    init_logger(tracing::Level::DEBUG);

    // Load config from file
    config::load("config.toml").await?;
    let config = config::get_config().await;

    // Generate key pair
    let keys = Arc::new(generate_key_pair());
    map::init_map();

    // Start HTTP API
    let http = build_http_server(SocketAddrV4::new(
        Ipv4Addr::from_str(&config.api.addr)?,
        config.api.port,
    ))?;

    tokio::spawn(async move { http.await });
    info!("API started on port {}", config.api.port);

    // Bind TCP server
    let listener = TcpListener::bind(SocketAddrV4::new(
        Ipv4Addr::from_str(&config.server.addr)?,
        config.server.port,
    ))
    .await?;
    info!("Server started on port {}", config.server.port);

    // Some graceful shutdown things
    let shutdown_signal = signal::ctrl_c();
    let shutdown_notify = Arc::new(Notify::new());

    // Graceful shutdown notifier task
    tokio::spawn({
        let shutdown_notify = shutdown_notify.clone();
        async move {
            match shutdown_signal.await {
                Ok(()) => {
                    info!("Shutdown signal received. Closing server...");
                    shutdown_notify.notify_one();
                }
                Err(err) => {
                    error!("Failed to listen for shutdown signal: {}", err);
                }
            }
        }
    });

    // TCP listener main loop
    loop {
        tokio::select! {
            result = listener.accept() => {
                match result {
                    Ok((stream, addr)) => {
                        let mut client = MinecraftServer::new(stream, keys.clone()).await?;
                        info!("New connection from: {}", addr);

                        tokio::spawn(async move {
                            // Setting client timeout
                            timeout(
                                Duration::from_secs(config.server.timeout),
                                client.run()
                            ).await.unwrap_or_else(|_| {error!("Connection from {} has been timed out", addr);})
                        });
                    },
                    Err(e) => {
                        error!("Failed to accept connection: {}", e);
                    }
                }
            },
            _ = shutdown_notify.notified() => {
                info!("Shutting down the server...");
                break;
            },
        }
    }

    Ok(())
}

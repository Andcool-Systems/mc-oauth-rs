/*
MIT License

Developed and implemented by AndcoolSystems, 2025
*/

mod api;
mod byte_buf_utils;
mod config;
mod encryption;
mod generators;
mod handlers;
mod logging;
mod map;
mod mojang;
mod packets;
mod responses;
mod session;

use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use api::build_http_server;
use generators::generate_key_pair;
use session::Session;
use tokio::{net::TcpListener, sync::Notify};
use tokio::{signal, time::timeout};
use tracing::{debug, error, info, Level};

use crate::logging::{init_logger, set_log_level, str_to_log_level};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up logger
    let reload_handle = init_logger(tracing::Level::DEBUG);

    // Load config from file
    config::load("config.toml").await?;
    let config = config::get_config().await;

    // Set logging level from config
    set_log_level(
        reload_handle,
        str_to_log_level(&config.global.logging_level).unwrap_or(Level::INFO),
    );

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
                        let mut client = Session::new(stream, keys.clone()).await;
                        info!("New connection from: {}", addr);

                        let timeout_duration = Duration::from_secs(config.server.timeout);
                        tokio::spawn(async move {
                            // Setting client timeout
                            match timeout(timeout_duration, client.run()).await {
                                Ok(Ok(_)) => {
                                    debug!("Connection from {} closed successfully", addr)
                                }
                                Ok(Err(e)) => {
                                    let _ = client
                                        .send_disconnect(config.messages.internal_error.clone())
                                        .await;
                                    error!("Internal error occurred: {}", e)
                                }
                                Err(e) => error!("Connection from {} has been timed out ({})", addr, e),
                            }
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

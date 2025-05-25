pub mod endpoint_handler;

use actix_web::{dev::Server, App, HttpServer};
use anyhow::Result;
use std::net::SocketAddrV4;

/// Build HTTP server
pub fn build_http_server(addr: SocketAddrV4) -> Result<Server> {
    let app = || App::new().service(endpoint_handler::code);

    Ok(HttpServer::new(app).bind(addr)?.run())
}

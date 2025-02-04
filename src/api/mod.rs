pub mod endpoint_handler;

use actix_web::{dev::Server, App, HttpServer};
use std::net::SocketAddrV4;

/// Сборка HTTP сервера
pub fn build_http_server(addr: SocketAddrV4) -> Server {
    let app = || App::new().service(endpoint_handler::code);

    HttpServer::new(app).bind(addr).unwrap().run()
}

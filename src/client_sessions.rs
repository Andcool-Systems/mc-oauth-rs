// use std::collections::HashMap;
// use std::net::SocketAddr;
// use std::sync::Arc;
// use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Session {
    pub server_id: String,
    pub proto_ver: Option<usize>,
    pub next_state: Option<usize>,
    pub nickname: Option<String>,
    pub secret: Option<Vec<u8>>, // Shared secret
}

impl Session {
    pub fn new() -> Self {
        Self {
            server_id: "mc-oauth-rs".to_string(),
            proto_ver: None,
            next_state: None,
            nickname: None,
            secret: None,
        }
    }
}

//pub type SessionMap = Arc<Mutex<HashMap<SocketAddr, Session>>>;

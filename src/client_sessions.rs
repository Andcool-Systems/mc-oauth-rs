use aes::Aes128;
use cfb8::Encryptor;
use uuid::Uuid;

use crate::generators::keys::generate_verify_token;

#[derive(Debug)]
pub enum NextStateEnum {
    Status,
    Login,
    Unknown,
}

#[derive(Debug)]
pub struct Session {
    pub server_id: String,
    pub proto_ver: Option<usize>,
    pub next_state: NextStateEnum,
    pub nickname: Option<String>,
    pub uuid: Option<Uuid>,
    pub secret: Option<Vec<u8>>, // Shared secret,
    pub verify_token: [u8; 4],
    pub cipher: Option<Encryptor<Aes128>>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            server_id: "mc-oauth-rs".to_string(),
            proto_ver: None,
            next_state: NextStateEnum::Unknown,
            nickname: None,
            uuid: None,
            secret: None,
            verify_token: generate_verify_token(),
            cipher: None,
        }
    }
}

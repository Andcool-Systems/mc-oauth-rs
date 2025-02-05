use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server_port: u16,
    pub api_port: u16,
    pub players_max: usize,
    pub players_online: usize,
    pub proto_ver: usize,
    pub server_ver: String,
    pub motd: String,
    pub code_life_time: u64,
}

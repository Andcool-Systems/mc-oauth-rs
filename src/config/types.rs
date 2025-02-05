use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Minecraft server port (Minecraft's default port is `25565`)
    pub server_port: u16,

    /// API port
    pub api_port: u16,

    /// Display count of max server players
    pub players_max: usize,

    /// Display count of max server players
    pub players_online: usize,

    /// Server protocol version (0 for auto)
    pub proto_ver: usize,

    /// Server version string (e.g. 1.20.5)
    pub server_ver: String,

    /// Server description (you can use a `§` codes)
    pub motd: String,

    /// Assigned 6-digit code life time in seconds
    pub code_life_time: u64,

    /// Path to the server icon
    pub icon: String,

    #[serde(skip)]
    /// Base 64 encoded server icon
    pub image: Option<String>,
}

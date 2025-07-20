use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    /// Global APP config
    pub global: GlobalConfig,

    /// API config
    pub api: API,

    /// Minecraft server config
    pub server: Server,

    /// Messages config
    pub messages: Messages,

    #[serde(skip)]
    /// Base 64 encoded server icon
    pub image: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GlobalConfig {
    /// Logging level
    pub logging_level: String,
}

#[derive(Deserialize, Debug)]
pub struct API {
    /// API address
    pub addr: String,

    /// API port
    pub port: u16,

    /// Life time of assigned code
    pub code_life_time: u64,

    /// Length of generated code
    pub code_length: u8,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    /// Server address
    pub addr: String,

    /// Server port
    pub port: u16,

    /// Server connection timeout
    pub timeout: u64,

    /// Minecraft server config
    pub config: ServerConfig,

    /// Server list ping config
    pub status: ServerStatus,

    /// Proxy addresses
    pub server_ips: Vec<String>,

    /// Enable partial support of 1.19 servers
    pub support_1_19: bool,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    /// Minecraft server name
    pub server_name: String,

    /// Protocol version (`0` for auto)
    pub protocol: usize,

    /// Minecraft version string (e.g. `1.20.1`)
    pub version: String,

    /// Session Auth URL  
    /// `{{NAME}}` in string will be replaced to client nickname  
    /// `{{HASH}}` will be replaced to generated client hash
    pub auth_url: String,
}

#[derive(Deserialize, Debug)]
pub struct ServerStatus {
    /// Server description (you can use MOTD)
    pub description: String,

    /// Max players count, displayed in server list
    pub players_max: usize,

    /// Online players count, displayed in server list
    pub players_online: usize,

    /// Path to the server icon
    pub icon_path: String,
}

#[derive(Deserialize, Debug)]
pub struct Messages {
    /// Message for success auth  
    /// `{{NAME}}` will be replaced to client nickname  
    /// `{{UUID}}` will be replaced to client UUID  
    /// `{{CODE}}` will be replaced to generated code
    pub success: String,

    /// Message for Mojang API error
    pub bad_session: String,

    /// Using a proxy
    pub using_proxy: String,

    /// Message for internal server error
    pub internal_error: String,

    /// Message for unsupported client version
    pub unsupported_client_version: String,
}

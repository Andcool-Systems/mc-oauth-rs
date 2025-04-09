pub mod server_icon;
pub mod types;

use anyhow::Result;
use std::sync::OnceLock;
use tokio::fs;
use tracing::warn;

static CONFIG: OnceLock<types::Config> = OnceLock::new();

#[inline]
pub async fn load(path: &str) -> Result<()> {
    let file = fs::read_to_string(path)
        .await
        .expect("Couldn't load config file");

    let mut config: types::Config = match toml::from_str(&file) {
        Ok(config) => config,
        Err(err) => {
            panic!("Couldn't parse config: {}", err.message())
        }
    };

    //let mut config: types::Config = toml::from_str(&file);
    match server_icon::load(&config.server.status.icon_path).await {
        Ok(base64) => config.image = Some(format!("data:image/png;base64,{}", base64)),
        Err(e) => warn!("Error loading server icon: {}", e),
    }

    CONFIG.set(config).expect("Couldn't load config");
    Ok(())
}

#[inline]
pub async fn get_config() -> &'static types::Config {
    CONFIG.get().expect("Config didn't loaded")
}

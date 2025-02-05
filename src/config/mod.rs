pub mod server_icon;
mod types;

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

    let mut config: types::Config =
        serde_json::from_str(&file).expect("Couldn't parse config json");
    match server_icon::load(&config.icon).await {
        Ok(base64) => config.image = Some(format!("data:image/png;base64,{}", base64)),
        Err(e) => warn!("Error loading server icon: {}", e)
    }

    CONFIG.set(config).expect("Couldn't load config");
    Ok(())
}

#[inline]
pub async fn get_config() -> &'static types::Config {
    CONFIG.get().expect("Config didn't loaded")
}

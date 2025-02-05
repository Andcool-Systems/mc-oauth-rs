mod types;

use anyhow::Result;
use std::sync::OnceLock;
use tokio::fs;

static CONFIG: OnceLock<types::Config> = OnceLock::new();

#[inline]
pub async fn load(path: &str) -> Result<()> {
    let file = fs::read_to_string(path)
        .await
        .expect("Couldn't load config file");
    CONFIG
        .set(serde_json::from_str(&file).expect("Couldn't parse config json"))
        .expect("Couldn't load config");
    Ok(())
}

#[inline]
pub async fn get_config() -> &'static types::Config {
    CONFIG.get().expect("Config didn't loaded")
}

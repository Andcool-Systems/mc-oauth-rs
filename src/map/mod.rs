use std::sync::{Arc, OnceLock};
use serde::Serialize;
use tokio::sync::{Mutex, MutexGuard};
pub mod expiring_map;

#[derive(Clone, Serialize)]
pub struct MinecraftData {
    pub name: String,
    pub uuid: String,
}

static MAP: OnceLock<Arc<Mutex<expiring_map::ExpiringMap<String, MinecraftData>>>> =
    OnceLock::new();

#[inline]
pub fn init_map() {
    MAP.set(Arc::new(Mutex::new(expiring_map::ExpiringMap::new())))
        .unwrap_or_else(|_| panic!("Cannot init hashmap!"));
}

#[inline]
pub async fn get_map() -> MutexGuard<'static, expiring_map::ExpiringMap<String, MinecraftData>> {
    MAP.get().expect("Map isn't initialized").lock().await
}

use std::sync::{Arc, OnceLock};
use tokio::sync::{Mutex, MutexGuard};

use crate::mojang::response::MojangResponse;
pub mod expiring_map;

static MAP: OnceLock<Arc<Mutex<expiring_map::ExpiringMap<String, MojangResponse>>>> =
    OnceLock::new();

#[inline]
pub fn init_map() {
    MAP.set(Arc::new(Mutex::new(expiring_map::ExpiringMap::new())))
        .unwrap_or_else(|_| panic!("Cannot init hashmap!"));
}

#[inline]
pub async fn get_map() -> MutexGuard<'static, expiring_map::ExpiringMap<String, MojangResponse>> {
    MAP.get().expect("Map isn't initialized").lock().await
}

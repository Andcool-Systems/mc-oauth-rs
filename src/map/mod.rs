use std::sync::OnceLock;

use crate::mojang::response::MojangResponse;
pub mod expiring_map;

static MAP: OnceLock<expiring_map::ExpiringMap<String, MojangResponse>> = OnceLock::new();

#[inline]
pub fn init_map() {
    MAP.set(expiring_map::ExpiringMap::new())
        .unwrap_or_else(|_| panic!("Cannot init hashmap!"));
}

#[inline]
pub async fn get_map() -> &'static expiring_map::ExpiringMap<String, MojangResponse> {
    MAP.get().expect("Map isn't initialized")
}

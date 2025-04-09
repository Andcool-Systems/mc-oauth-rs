use std::sync::OnceLock;
pub mod expiring_map;

static MAP: OnceLock<expiring_map::ExpiringMap> = OnceLock::new();

#[inline]
pub fn init_map() {
    MAP.set(expiring_map::ExpiringMap::new())
        .unwrap_or_else(|_| panic!("Cannot init hashmap!"));
}

#[inline]
pub async fn get_map() -> &'static expiring_map::ExpiringMap {
    MAP.get().expect("Map isn't initialized")
}

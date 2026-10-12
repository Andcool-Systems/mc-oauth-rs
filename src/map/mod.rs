use std::sync::OnceLock;
pub mod expiring_map;

static MAP: OnceLock<expiring_map::ExpiringMap> = OnceLock::new();

#[inline]
pub async fn get_map() -> &'static expiring_map::ExpiringMap {
    MAP.get_or_init(expiring_map::ExpiringMap::new)
}

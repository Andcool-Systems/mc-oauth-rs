use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::RwLock,
    time::{sleep, Duration},
};

pub struct ExpiringMap<K, V>
where
    K: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    pub map: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> ExpiringMap<K, V>
where
    K: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, key: K, value: V, ttl: Duration) {
        self.map.write().await.insert(key.clone(), value);

        let map = Arc::clone(&self.map);
        tokio::spawn(async move {
            sleep(ttl).await;
            map.write().await.remove(&key);
        });
    }
}

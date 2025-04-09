use crate::mojang::response::MojangResponse;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::RwLock,
    time::{sleep, Duration},
};

pub struct ExpiringMap {
    pub map: Arc<RwLock<HashMap<String, MojangResponse>>>,
}

impl ExpiringMap {
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, key: String, value: MojangResponse, ttl: Duration) {
        self.map.write().await.insert(key.clone(), value);

        let map = Arc::clone(&self.map);
        tokio::spawn(async move {
            sleep(ttl).await;
            map.write().await.remove(&key);
        });
    }
}

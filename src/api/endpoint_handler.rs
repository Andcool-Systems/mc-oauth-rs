use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use tracing::{info, instrument};

use crate::map::get_map;

#[instrument(skip(path))]
#[get("/code/{id}")]
pub async fn code(path: web::Path<String>) -> impl Responder {
    // Get data endpoint

    let path = path.into_inner();
    info!("Get code with path: {}", path);

    let map_guard = get_map().await;
    let value = {
        let map = map_guard.map.read().await;
        map.get(&path).cloned()
    };

    match value {
        Some(value) => {
            let mut map = map_guard.map.write().await;
            map.remove(&path); // Remove data after first use

            HttpResponse::Ok().json(serde_json::to_value(&value).unwrap())
        }
        None => HttpResponse::NotFound().json(json!({"message": "Code not found"})),
    }
}

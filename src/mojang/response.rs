use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MojangResponse {
    pub id: String,
    pub name: String,
}

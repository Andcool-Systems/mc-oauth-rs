use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct MojangResponse {
    #[serde(rename(serialize = "UUID"))]
    pub id: String,
    #[serde(rename(serialize = "nickname"))]
    pub name: String,
    pub properties: Vec<Properties>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct Properties {
    name: String,
    value: String,
    signature: String,
}

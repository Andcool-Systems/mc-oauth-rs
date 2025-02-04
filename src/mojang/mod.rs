pub mod response;

use std::sync::Arc;

use anyhow::Result;
use hex::encode;
use sha1::{Digest, Sha1};

use crate::client_sessions::Session;
use rsa::pkcs8::EncodePublicKey;

pub async fn join(
    session: &mut Session,
    keys: Arc<rsa::RsaPrivateKey>,
) -> Result<Option<response::MojangResponse>> {
    let mut digest = Sha1::new();
    digest.update(session.server_id.as_bytes());
    digest.update(session.secret.clone().unwrap().as_slice());
    digest.update(keys.to_public_key().to_public_key_der()?);
    let hash = encode(digest.finalize());

    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
        session.nickname.clone().unwrap(),
        hash
    );

    let response = reqwest::get(url.clone()).await?;

    if response.status().as_u16() != 200 {
        return Ok(None);
    }

    Ok(Some(serde_json::from_str(&response.text().await?)?))
}

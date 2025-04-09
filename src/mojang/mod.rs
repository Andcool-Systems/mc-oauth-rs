pub mod response;
use crate::{client::Session, config};
use anyhow::Result;
use num_bigint::BigInt;
use rsa::pkcs8::EncodePublicKey;
use sha1::{Digest, Sha1};
use std::sync::Arc;
use tracing::debug;

pub async fn join(
    session: &mut Session,
    keys: Arc<rsa::RsaPrivateKey>,
) -> Result<Option<response::MojangResponse>> {
    let mut digest = Sha1::new();
    digest.update(session.server_id.as_bytes());
    digest.update(session.secret.clone().unwrap().as_slice());
    digest.update(keys.to_public_key().to_public_key_der()?.as_bytes());
    let hash = BigInt::from_signed_bytes_be(&digest.finalize()).to_str_radix(16);

    debug!("Creating request to Mojang API with hash: {hash}");

    let config = config::get_config().await;
    let response = reqwest::get(
        config
            .server
            .config
            .auth_url
            .replace("{{NAME}}", &session.nickname.clone().unwrap())
            .replace("{{HASH}}", &hash),
    )
    .await?;

    debug!(
        "Mojang API responded with code: {}",
        response.status().as_u16()
    );

    if response.status().as_u16() != 200 {
        return Ok(None);
    }

    Ok(Some(serde_json::from_str(&response.text().await?)?))
}

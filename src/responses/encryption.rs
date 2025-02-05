use anyhow::Result;
use rsa::pkcs8::EncodePublicKey;
use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{client_sessions::Session, packets::encryption_request::EncryptionRequestPacket};

pub async fn send(
    stream: &mut TcpStream,
    keys: Arc<rsa::RsaPrivateKey>,
    session: &mut Session,
) -> Result<()> {
    let public_der = keys
        .to_public_key()
        .to_public_key_der()
        .expect("Failed to export public key")
        .into_vec();

    let should_authenticate = if session.proto_ver.unwrap() >= 766 {
        Some(true)
    } else {
        None
    };

    let packet = EncryptionRequestPacket::new(
        session.server_id.clone(),
        public_der,
        session.verify_token.to_vec(),
        should_authenticate,
    );
    stream.writable().await?;
    stream.write_all(&packet.build()?).await?;
    Ok(())
}

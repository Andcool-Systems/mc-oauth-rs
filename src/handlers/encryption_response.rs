use std::sync::Arc;

use crate::{client_sessions::Session, packets::encryption_response::EncryptionResponsePacket};
use anyhow::Result;
use bytes::BytesMut;

use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

pub fn handle(
    session: &mut Session,
    buffer: &mut BytesMut,
    keys: Arc<rsa::RsaPrivateKey>,
) -> Result<()> {
    let response = EncryptionResponsePacket::parse(buffer)?;

    let decrypted_secret = decrypt(keys.as_ref(), &response.shared_secret)?;
    let decrypted_verify = decrypt(keys.as_ref(), &response.verify_token)?;

    if decrypted_verify
        .iter()
        .zip(&session.verify_token)
        .filter(|&(a, b)| a == b)
        .count()
        != decrypted_verify.len()
    {
        panic!("Verify tokens didn't match!");
    }

    session.secret = Some(decrypted_secret.clone());
    Ok(())
}

fn decrypt(private_key: &RsaPrivateKey, data: &Vec<u8>) -> Result<Vec<u8>, rsa::Error> {
    return private_key.decrypt(Pkcs1v15Encrypt, data);
}

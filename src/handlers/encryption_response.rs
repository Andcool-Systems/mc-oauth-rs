use std::sync::Arc;

use crate::{client::Session, packets::encryption_response::EncryptionResponsePacket};
use aes::{cipher::KeyIvInit, Aes128};
use anyhow::Result;
use bytes::BytesMut;

use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

pub fn handle_encryption(
    session: &mut Session,
    buffer: &mut BytesMut,
    keys: Arc<rsa::RsaPrivateKey>,
) -> Result<()> {
    let response = EncryptionResponsePacket::parse(buffer)?;

    // Decrypt client's keys
    let decrypted_secret = decrypt(&keys, &response.shared_secret)?;
    let decrypted_verify = decrypt(&keys, &response.verify_token)?;

    // Check tokens equality
    if decrypted_verify
        .iter()
        .zip(&session.verify_token)
        .filter(|&(a, b)| a == b)
        .count()
        != decrypted_verify.len()
    {
        panic!("Verify tokens didn't match!");
    }

    // Set up client cipher
    session.secret = Some(decrypted_secret.clone());
    session.cipher = Some(
        cfb8::Encryptor::<Aes128>::new_from_slices(
            &decrypted_secret.clone(),
            &decrypted_secret.clone(),
        )
        .unwrap(),
    );
    Ok(())
}

fn decrypt(private_key: &RsaPrivateKey, data: &Vec<u8>) -> Result<Vec<u8>, rsa::Error> {
    return private_key.decrypt(Pkcs1v15Encrypt, data);
}

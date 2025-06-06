use crate::{packets::encryption_response::EncryptionResponsePacket, session::Session};
use aes::{cipher::KeyIvInit, Aes128};
use anyhow::{Error, Result};
use rsa::Pkcs1v15Encrypt;

impl Session {
    /**
    Handle encryption response

    Check verify tokens and set up the cipher
    */
    pub fn handle_encryption(&mut self) -> Result<()> {
        let response = EncryptionResponsePacket::parse(self)?;

        // Decrypt client's keys
        let decrypted_secret = self
            .keys
            .decrypt(Pkcs1v15Encrypt, &response.shared_secret)?;

        if response.has_verify {
            let decrypted_verify = self.keys.decrypt(Pkcs1v15Encrypt, &response.verify_token)?;

            // Check tokens equality
            if decrypted_verify
                .iter()
                .zip(&self.verify_token)
                .filter(|&(a, b)| a == b)
                .count()
                != decrypted_verify.len()
            {
                return Err(Error::msg("Verify tokens didn't match!"));
            }
        }

        // Set up client cipher
        self.secret = Some(decrypted_secret.clone());
        self.cipher = Some(cfb8::Encryptor::<Aes128>::new_from_slices(
            &decrypted_secret.clone(),
            &decrypted_secret.clone(),
        )?);
        Ok(())
    }
}

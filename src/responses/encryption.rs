use crate::{packets::encryption_request::EncryptionRequestPacket, session::Session};
use anyhow::Result;
use rsa::pkcs8::EncodePublicKey;
use tokio::io::AsyncWriteExt;

impl Session {
    /**
    Send encryption response
    */
    pub async fn send_encryption(&mut self) -> Result<()> {
        let public_der = self.keys.to_public_key().to_public_key_der()?.into_vec();

        let should_authenticate = if self.proto_ver >= 766 {
            Some(true)
        } else {
            None
        };

        let packet = EncryptionRequestPacket::new(
            self.server_id.clone(),
            public_der,
            self.verify_token.to_vec(),
            should_authenticate,
        );
        self.stream.writable().await?;
        self.stream.write_all(&packet.build()?).await?;
        Ok(())
    }
}

use crate::{packets::encryption_request::EncryptionRequestPacket, server::MinecraftServer};
use anyhow::Result;
use rsa::pkcs8::EncodePublicKey;
use tokio::io::AsyncWriteExt;

impl MinecraftServer {
    /**
    Send encryption response
    */
    pub async fn send_encryption(&mut self) -> Result<()> {
        let public_der = self.keys.to_public_key().to_public_key_der()?.into_vec();

        let proto_ver = match self.session.proto_ver {
            Some(x) => x,
            None => unreachable!(),
        };
        let should_authenticate = if proto_ver >= 766 { Some(true) } else { None };

        let packet = EncryptionRequestPacket::new(
            self.session.server_id.clone(),
            public_der,
            self.session.verify_token.to_vec(),
            should_authenticate,
        );
        self.stream.writable().await?;
        self.stream.write_all(&packet.build()?).await?;
        Ok(())
    }
}

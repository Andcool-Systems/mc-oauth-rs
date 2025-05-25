use crate::server::MinecraftServer;
use aes::cipher::AsyncStreamCipher;
use bytes::BytesMut;

impl MinecraftServer {
    /**
    Encrypt minecraft packet in place

    If session cipher is not set — skip the encryption
    */
    pub fn encrypt_packet(&mut self, data: &mut BytesMut) {
        if self.session.cipher.is_none() {
            return;
        }

        if let Some(cipher) = &mut self.session.cipher {
            cipher.clone().encrypt(data);
        }
    }
}

use crate::session::Session;
use aes::cipher::AsyncStreamCipher;
use bytes::BytesMut;

impl Session {
    /**
    Encrypt minecraft packet in place

    If session cipher is not set — skip the encryption
    */
    pub fn encrypt_packet(&mut self, data: &mut BytesMut) {
        if self.cipher.is_none() {
            return;
        }

        if let Some(cipher) = &mut self.cipher {
            cipher.clone().encrypt(data);
        }
    }
}

use crate::client::Session;
use bytes::BytesMut;

pub fn encrypt_packet(data: &mut BytesMut, session: &mut Session) {
    use aes::cipher::{generic_array::GenericArray, BlockEncryptMut, BlockSizeUser};
    use aes::Aes128;

    if session.cipher.is_none() {
        return;
    }

    let mut cipher = session.cipher.clone().unwrap();
    for chunk in data.chunks_mut(cfb8::Encryptor::<Aes128>::block_size()) {
        let gen_arr = GenericArray::from_mut_slice(chunk);
        cipher.encrypt_block_mut(gen_arr);
    }
}

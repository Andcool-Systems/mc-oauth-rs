use bytes::BytesMut;

pub fn encrypt_packet(data: &mut BytesMut, key: Vec<u8>) {
    use aes::cipher::{generic_array::GenericArray, BlockEncryptMut, BlockSizeUser, KeyIvInit};
    use aes::Aes128;

    pub type Aes128CfbEnc = cfb8::Encryptor<Aes128>;
    let mut cipher = Aes128CfbEnc::new_from_slices(&key, &key).unwrap();

    for chunk in data.chunks_mut(Aes128CfbEnc::block_size()) {
        let gen_arr = GenericArray::from_mut_slice(chunk);
        cipher.encrypt_block_mut(gen_arr);
    }
}

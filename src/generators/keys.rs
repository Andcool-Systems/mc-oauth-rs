use rand::{rngs::OsRng, Rng};
use rsa::RsaPrivateKey;

pub fn generate_key_pair() -> RsaPrivateKey {
    let bits = 1024;
    RsaPrivateKey::new(&mut OsRng, bits).expect("failed to generate a key")
}

pub fn generate_verify_token() -> [u8; 4] {
    let mut token = [0u8; 4];
    rand::thread_rng().fill(&mut token);
    token
}

pub fn generate_code() -> String {
    (0..6)
        .map(|_| rand::thread_rng().gen_range(0..=9).to_string())
        .collect()
}

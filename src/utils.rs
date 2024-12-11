use addr::parse_domain_name;

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng as AesOsRng},
    Aes256Gcm, Key, Nonce
};
use base64::{Engine as _, engine::general_purpose};

const NONCE_LENGTH: usize = 12;


//argon2 salting to turn master password into key
pub fn derive_key(master_password: &str) -> [u8; 32] {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(master_password.as_bytes(), &salt).unwrap();
    let binding = password_hash.hash.unwrap();
    let hash_bytes = binding.as_bytes();
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash_bytes[..32]);
    key
}

//Aes256 to encrypt/decrypt password
pub fn encrypt_password(password: &str, master_key: &[u8; 32]) -> String {
    let key = Key::<Aes256Gcm>::from_slice(master_key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut AesOsRng);
    let ciphertext = cipher.encrypt(&nonce, password.as_bytes().as_ref())
        .expect("Encryption failed");
    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);
    general_purpose::STANDARD_NO_PAD.encode(combined)
}

pub fn decrypt_password(encrypted: &str, master_key: &[u8; 32]) -> String {
    let key = Key::<Aes256Gcm>::from_slice(master_key);
    let cipher = Aes256Gcm::new(key);
    let decoded = general_purpose::STANDARD_NO_PAD.decode(encrypted)
        .expect("Decoding failed");
    let nonce = Nonce::from_slice(&decoded[..NONCE_LENGTH]);
    let plaintext = cipher.decrypt(nonce, decoded[NONCE_LENGTH..].as_ref())
        .expect("Decryption failed");
    String::from_utf8(plaintext).expect("Invalid UTF-8")
}



pub fn parse_domain(url: &str) -> Option<String> {
    let domain = url.trim_start_matches("http://")
                    .trim_start_matches("https://")
                    .trim_start_matches("www.");

    let end = domain.find('/').unwrap_or(domain.len());
    let domain = &domain[..end];

    parse_domain_name(domain)
        .ok()
        .and_then(|parsed| parsed.root().map(String::from))
}   


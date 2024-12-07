use addr::parse_domain_name;

use linked_hash_map::LinkedHashMap;
use std::error::Error;
use csv::{ReaderBuilder, WriterBuilder};
use crate::LoginDatabase;

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng as AesOsRng},
    Aes256Gcm, Key, Nonce
};
use base64::{Engine as _, engine::general_purpose};

const NONCE_LENGTH: usize = 12;

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

pub fn read_csv(file_path: &str) -> Result<LoginDatabase, Box<dyn Error>> {
    let mut logins = LinkedHashMap::new();
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    for result in reader.records() {
        let record = result?;
        if record.len() == 3 {
            logins.insert(
                record[0].to_string(),
                (record[1].to_string(), record[2].to_string()),
            );
        }
    }

    Ok(logins)
}

pub fn write_csv(file_path: &str, logins: &LoginDatabase) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    // Write in the correct order: domain, username, password
    for (domain, (username, password)) in logins {
        writer.write_record(&[domain, username, password])?;
    }
    writer.flush()?;

    Ok(())
}

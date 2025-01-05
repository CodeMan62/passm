use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Argon2, PasswordHasher};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::error::Error;

const SALT_LENGTH: usize = 16;
const NONCE_LENGTH: usize = 12;

#[derive(Serialize, Deserialize)]
pub struct EncryptedData {
    salt: String,
    nonce: String,
    ciphertext: String,
}

pub struct Encryptor {
    key: Key<Aes256Gcm>,
    master_password: String,
}

impl Encryptor {
    pub fn new(master_password: &str) -> Result<Self, Box<dyn Error>> {
        let salt = Encryptor::generate_random_bytes(SALT_LENGTH);
        let key = Encryptor::derive_key(master_password, &salt)?;
        Ok(Self {
            key,
            master_password: master_password.to_string(),
        })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<EncryptedData, Box<dyn Error>> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Encryptor::generate_random_bytes(NONCE_LENGTH);
        let nonce = Nonce::from_slice(&nonce);

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        Ok(EncryptedData {
            salt: general_purpose::STANDARD_NO_PAD.encode(&self.key),
            nonce: general_purpose::STANDARD_NO_PAD.encode(nonce),
            ciphertext: general_purpose::STANDARD_NO_PAD.encode(&ciphertext),
        })
    }

    pub fn decrypt(&self, encrypted_data: &EncryptedData) -> Result<String, Box<dyn Error>> {
        let salt = general_purpose::STANDARD_NO_PAD.decode(&encrypted_data.salt)?;
        let key = Encryptor::derive_key(&self.master_password, &salt)?;
        let cipher = Aes256Gcm::new(&key);

        let nonce = general_purpose::STANDARD_NO_PAD.decode(&encrypted_data.nonce)?;
        let nonce = Nonce::from_slice(&nonce);

        let ciphertext = general_purpose::STANDARD_NO_PAD.decode(&encrypted_data.ciphertext)?;

        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| format!("Decryption failed: {}", e))?;

        String::from_utf8(plaintext).map_err(|e| e.into())
    }

    fn derive_key(password: &str, salt: &[u8]) -> Result<Key<Aes256Gcm>, Box<dyn Error>> {
        let argon2 = Argon2::default();
        let salt = argon2::password_hash::SaltString::encode_b64(salt)
            .map_err(|e| format!("Salt encoding failed: {}", e))?;
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Password hashing failed: {}", e))?;

        let hash = password_hash.hash.unwrap();
        let hash_bytes = hash.as_bytes();
        let key = Key::<Aes256Gcm>::from_slice(hash_bytes);
        Ok(*key)
    }

    fn generate_random_bytes(length: usize) -> Vec<u8> {
        (0..length).map(|_| rand::random::<u8>()).collect()
    }

    pub fn get_master_password(&self) -> &str {
        &self.master_password
    }
}


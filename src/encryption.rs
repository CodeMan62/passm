use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose, Engine as _};
use rand_core::RngCore;

pub struct Encryption {
    cipher: Aes256Gcm,
}

impl Encryption {
    pub fn new(master_password: &str) -> Result<Self> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(master_password.as_bytes(), &salt)
            .map_err(|e| anyhow!("Failed to hash master password: {}", e))?;
        let key = password_hash.hash.unwrap();
        let cipher = Aes256Gcm::new_from_slice(&key.as_bytes()[..32])
            .map_err(|e| anyhow!("Failed to create AES-GCM cipher: {}", e))?;
        Ok(Self { cipher })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        let nonce_bytes = Encryption::generate_nonce();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        let mut combined = nonce.to_vec();
        combined.extend_from_slice(&ciphertext);
        Ok(general_purpose::URL_SAFE_NO_PAD.encode(combined))
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String> {
        let decoded = general_purpose::URL_SAFE_NO_PAD
            .decode(ciphertext)
            .map_err(|e| anyhow!("Base64 decoding failed: {}", e))?;

        if decoded.len() < 12 {
            return Err(anyhow!("Invalid ciphertext"));
        }

        let (nonce, ciphertext) = decoded.split_at(12);
        let plaintext = self
            .cipher
            .decrypt(Nonce::from_slice(nonce), ciphertext)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;

        String::from_utf8(plaintext).map_err(|e| anyhow!("UTF-8 decoding failed: {}", e))
    }

    fn generate_nonce() -> [u8; 12] {
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        nonce
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let master_password = "super_secret_password";
        let encryption = Encryption::new(master_password).unwrap();

        let plaintext = "This is a secret message";
        let encrypted = encryption.encrypt(plaintext).unwrap();
        let decrypted = encryption.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_different_passwords() {
        let encryption1 = Encryption::new("password1").unwrap();
        let encryption2 = Encryption::new("password2").unwrap();

        let plaintext = "Secret message";
        let encrypted = encryption1.encrypt(plaintext).unwrap();

        assert!(encryption2.decrypt(&encrypted).is_err());
    }
}


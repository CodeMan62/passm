use crate::encryption::{EncryptedData, Encryptor};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const VAULT_FILE: &str = "vault.json";

#[derive(Serialize, Deserialize)]
pub struct PasswordEntry {
    service: String,
    username: String,
    password: EncryptedData,
    last_modified: DateTime<Utc>,
}

pub struct Storage {
    vault: HashMap<String, PasswordEntry>,
    encryptor: Encryptor,
}

impl Storage {
    pub fn new(master_password: &str) -> Result<Self, Box<dyn Error>> {
        let encryptor = Encryptor::new(master_password)?;
        let vault = if Path::new(VAULT_FILE).exists() {
            Storage::load_vault(master_password)?
        } else {
            HashMap::new()
        };
        Ok(Self { vault, encryptor })
    }

    pub fn add_password(
        &mut self,
        service: &str,
        username: &str,
        password: &str,
    ) -> Result<(), Box<dyn Error>> {
        let encrypted_password = self.encryptor.encrypt(password)?;
        let entry = PasswordEntry {
            service: service.to_string(),
            username: username.to_string(),
            password: encrypted_password,
            last_modified: Utc::now(),
        };
        self.vault.insert(service.to_string(), entry);
        self.save_vault()
    }

    pub fn get_password(&self, service: &str) -> Result<String, Box<dyn Error>> {
        let entry = self
            .vault
            .get(service)
            .ok_or_else(|| format!("No password found for service: {}", service))?;
        self.encryptor.decrypt(&entry.password)
    }

    pub fn list_services(&self) -> Vec<(String, String, DateTime<Utc>)> {
        self.vault
            .iter()
            .map(|(service, entry)| {
                (
                    service.clone(),
                    entry.username.clone(),
                    entry.last_modified,
                )
            })
            .collect()
    }

    pub fn remove_password(&mut self, service: &str) -> Result<(), Box<dyn Error>> {
        self.vault
            .remove(service)
            .ok_or_else(|| format!("No password found for service: {}", service))?;
        self.save_vault()
    }

    fn save_vault(&self) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&self.vault)?;
        let mut file = File::create(VAULT_FILE)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn load_vault(master_password: &str) -> Result<HashMap<String, PasswordEntry>, Box<dyn Error>> {
        let mut file = File::open(VAULT_FILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let vault: HashMap<String, PasswordEntry> = serde_json::from_str(&contents)?;
        
        // Verify the master password by trying to decrypt one entry
        if let Some((_, entry)) = vault.iter().next() {
            let encryptor = Encryptor::new(master_password)?;
            encryptor.decrypt(&entry.password)?;
        }
        
        Ok(vault)
    }
}


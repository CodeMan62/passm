use std::fmt;

#[derive(Debug)]
pub enum PasswordManagerError {
    VaultNotInitialized,
    VaultAlreadyExists,
    ServiceNotFound(String),
    InvalidPassword,
    IoError(std::io::Error),
    EncryptionError(String),
}

impl fmt::Display for PasswordManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VaultNotInitialized => write!(f, "Vault is not initialized. Run 'init' first."),
            Self::VaultAlreadyExists => write!(f, "Vault already exists."),
            Self::ServiceNotFound(s) => write!(f, "Service '{}' not found in vault.", s),
            Self::InvalidPassword => write!(f, "Invalid master password."),
            Self::IoError(e) => write!(f, "IO Error: {}", e),
            Self::EncryptionError(e) => write!(f, "Encryption Error: {}", e),
        }
    }
}

impl std::error::Error for PasswordManagerError {}


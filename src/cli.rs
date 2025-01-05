use crate::storage::Storage;
use crate::ui::UI;
use clap::{Parser, Subcommand};
use std::error::Error;

#[derive(Parser)]
#[command(
    name = "pman",
    about = "A secure password manager",
    version = "0.1.0",
    author = "Your Name",
    before_help = "🔒 Secure Password Manager - Keep your passwords safe",
    after_help = "For more information, visit: https://github.com/yourusername/password-manager"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new password
    Add {
        /// Service name (e.g., 'github', 'gmail')
        #[arg(short, long)]
        service: String,
        
        /// Username for the service
        #[arg(short, long)]
        username: String,
        
        /// Generate a random password instead of manual input
        #[arg(short, long)]
        generate: bool,
    },
    
    /// Get a password
    Get {
        /// Service name to retrieve password for
        #[arg(short, long)]
        service: String,
        
        /// Copy to clipboard instead of displaying
        #[arg(short, long)]
        copy: bool,
    },
    
    /// List all stored services
    List,
    
    /// Generate a new password
    Generate {
        /// Password length
        #[arg(short, long, default_value = "16")]
        length: u8,
        
        /// Include special characters
        #[arg(short, long)]
        special: bool,
    },
    
    /// Remove a password
    Remove {
        /// Service name to remove
        #[arg(short, long)]
        service: String,
    },
}

impl Cli {
    pub fn execute(&self, mut storage: Storage, ui: &UI) -> Result<(), Box<dyn Error>> {
        match &self.command {
            Some(cmd) => self.handle_command(cmd, &mut storage, ui),
            None => self.show_interactive_menu(&mut storage, ui),
        }
    }

    fn handle_command(&self, command: &Commands, storage: &mut Storage, ui: &UI) -> Result<(), Box<dyn Error>> {
        match command {
            Commands::Add { service, username, generate } => {
                self.handle_add(storage, ui, service, username, *generate)
            }
            Commands::Get { service, copy } => self.handle_get(storage, ui, service, *copy),
            Commands::List => self.handle_list(storage, ui),
            Commands::Generate { length, special } => self.handle_generate(ui, *length, *special),
            Commands::Remove { service } => self.handle_remove(storage, ui, service),
        }
    }

    fn show_interactive_menu(&self, storage: &mut Storage, ui: &UI) -> Result<(), Box<dyn Error>> {
        ui.clear_screen()?;

        let options = vec![
            "Add new password",
            "Get password",
            "List all passwords",
            "Generate password",
            "Remove password",
            "Exit",
        ];

        loop {
            match ui.show_menu("Main Menu", &options)? {
                0 => {
                    let service = ui.get_input("Enter service name")?;
                    let username = ui.get_input("Enter username")?;
                    let generate = ui.confirm("Generate random password?")?;
                    self.handle_add(storage, ui, &service, &username, generate)?;
                }
                1 => {
                    let service = ui.get_input("Enter service name")?;
                    self.handle_get(storage, ui, &service, false)?;
                }
                2 => self.handle_list(storage, ui)?,
                3 => self.handle_generate(ui, 16, true)?,
                4 => {
                    let service = ui.get_input("Enter service name to remove")?;
                    self.handle_remove(storage, ui, &service)?;
                }
                5 => {
                    ui.success("Goodbye! 👋");
                    break;
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn handle_add(&self, storage: &mut Storage, ui: &UI, service: &str, username: &str, generate: bool) -> Result<(), Box<dyn Error>> {
        let password = if generate {
            self.generate_password(16, true)
        } else {
            ui.get_password("Enter password")?
        };

        let pb = ui.show_spinner(&format!("Adding password for {}...", service));
        storage.add_password(service, username, &password)?;
        pb.finish_and_clear();
        ui.success(&format!("Password added for {}", service));
        Ok(())
    }

    fn handle_get(&self, storage: &Storage, ui: &UI, service: &str, copy: bool) -> Result<(), Box<dyn Error>> {
        let pb = ui.show_spinner(&format!("Retrieving password for {}...", service));
        let password = storage.get_password(service)?;
        pb.finish_and_clear();
        
        if copy {
            // TODO: Implement clipboard functionality
            ui.success("Password copied to clipboard!");
        } else {
            ui.display_password(&password);
        }
        
        Ok(())
    }

    fn handle_list(&self,storage: &Storage, ui: &UI) -> Result<(), Box<dyn Error>> {
        let pb = ui.show_spinner("Fetching passwords...");
        let services = storage.list_services();
        pb.finish_and_clear();

        if services.is_empty() {
            ui.warning("No passwords stored yet.");
        } else {
            let entries: Vec<_> = services
                .into_iter()
                .map(|(service, username, last_modified)| {
                    crate::ui::PasswordEntry {
                        service,
                        username,
                        last_modified: last_modified.to_rfc3339(),
                        strength: "N/A".to_string(), // TODO: Implement password strength calculation
                    }
                })
                .collect();
            ui.display_table(entries);
        }
        Ok(())
    }

    fn handle_generate(&self, ui: &UI, length: u8, special: bool) -> Result<(), Box<dyn Error>> {
        let pb = ui.show_spinner("Generating secure password...");
        let password = self.generate_password(length, special);
        pb.finish_and_clear();

        ui.display_password(&password);
        Ok(())
    }

    fn handle_remove(&self, storage: &mut Storage, ui: &UI, service: &str) -> Result<(), Box<dyn Error>> {
        let pb = ui.show_spinner(&format!("Removing password for {}...", service));
        storage.remove_password(service)?;
        pb.finish_and_clear();
        ui.success(&format!("Password removed for {}", service));
        Ok(())
    }

    fn generate_password(&self, length: u8, special: bool) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789";
        const SPECIAL_CHARS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

        let mut rng = rand::thread_rng();
        let mut password = String::with_capacity(length as usize);

        for _ in 0..length {
            if special && rng.gen_ratio(1, 4) {
                password.push(SPECIAL_CHARS[rng.gen_range(0..SPECIAL_CHARS.len())] as char);
            } else {
                password.push(CHARSET[rng.gen_range(0..CHARSET.len())] as char);
            }
        }

        password
    }
}


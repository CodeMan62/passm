use colored::*;
use console::{Emoji, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tabled::{Table, Tabled};

static LOCK: Emoji<'_, '_> = Emoji("🔒 ", "");
static KEY: Emoji<'_, '_> = Emoji("🔑 ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");

#[derive(Tabled)]
pub struct PasswordEntry {
    #[tabled(rename = "Service")]
    pub service: String,
    #[tabled(rename = "Username")]
    pub username: String,
    #[tabled(rename = "Last Modified")]
    pub last_modified: String,
    #[tabled(rename = "Strength")]
    pub strength: String,
}

pub struct UI {
    term: Term,
    theme: ColorfulTheme,
}

impl UI {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
            theme: ColorfulTheme::default(),
        }
    }

    pub fn clear_screen(&self) -> std::io::Result<()> {
        self.term.clear_screen()?;
        Ok(())
    }

    pub fn show_spinner(&self, message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .template("{spinner:.blue} {msg}")
                .unwrap(),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    pub fn success(&self, message: &str) {
        println!("\n{} {}", SPARKLE, message.bright_green());
    }

    pub fn warning(&self, message: &str) {
        println!("\n⚠️  {}", message.yellow());
    }

    pub fn error(&self, message: &str) {
        println!("\n❌ {}", message.bright_red());
    }

    pub fn get_password(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(Password::with_theme(&self.theme)
            .with_prompt(prompt)
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.len() < 8 {
                    Err("Password must be at least 8 characters long")
                } else {
                    Ok(())
                }
            })
            .interact()?)
    }

    pub fn confirm(&self, prompt: &str) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(Confirm::with_theme(&self.theme)
            .with_prompt(prompt)
            .default(false)
            .interact()?)
    }

    pub fn show_menu(&self, title: &str, options: &[&str]) -> Result<usize, Box<dyn std::error::Error>> {
        println!("\n{} {}\n", LOCK, title.bright_blue().bold());
        Ok(Select::with_theme(&self.theme)
            .items(options)
            .default(0)
            .interact()?)
    }

    pub fn display_table(&self, entries: Vec<PasswordEntry>) {
        let table = Table::new(entries).to_string();
        println!("\n{}", table);
    }

    pub fn get_input(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(Input::<String>::with_theme(&self.theme)
            .with_prompt(prompt)
            .interact()?)
    }

    pub fn display_password(&self, password: &str) {
        println!("\n{} Password: {}", KEY, password.bright_green());
    }
}


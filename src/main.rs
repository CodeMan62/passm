mod vault;
mod ui;
mod encryption;

use colored::*;
use console::{Term, Key};
use vault::{create, view, generate, analyze};
use anyhow::Result;

fn main() -> Result<()> {
    let term = Term::stdout();
    println!("{}", ui::BANNER);
    println!("{}", "Welcome to Vault - Your Secure Password Sanctuary".bright_green());

    // For demonstration purposes, we'll use a hardcoded master password
    // In a real application, you'd want to securely obtain this from the user
    let master_password = "example_master_password";

    loop {
        ui::display_menu(&term);

        if let Ok(key) = term.read_key() {
            match key {
                Key::Char('1') => create::run(master_password).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                Key::Char('2') => view::run()?,
                Key::Char('3') => generate::run()?,
                Key::Char('4') => analyze::run()?,
                Key::Char('q') | Key::Char('Q') => {
                    ui::display_exit_animation();
                    break;
                },
                _ => println!("{}", "Invalid option. Please try again.".yellow()),
            }
        }
    }

    Ok(())
}



mod vault;
mod ui;

use colored::*;
use console::{Term, Key};
use vault::{create, view, generate, analyze};

fn main() {
    let term = Term::stdout();
    println!("{}", ui::BANNER);
    println!("{}", "Welcome to Vault - Your Secure Password Sanctuary".bright_green());

    loop {
        ui::display_menu(&term);
        
        if let Ok(key) = term.read_key() {
            match key {
                Key::Char('1') => create::run(),
                Key::Char('2') => view::run(),
                Key::Char('3') => generate::run(),
                Key::Char('4') => analyze::run(),
                Key::Char('q') | Key::Char('Q') => {
                    ui::display_exit_animation();
                    break;
                },
                _ => println!("{}", "Invalid option. Please try again.".yellow()),
            }
        }
    }
}



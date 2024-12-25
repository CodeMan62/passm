use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn run() {
    println!("{}", "Viewing passwords".blue());
    
    // This is a mock-up of stored passwords
    let passwords = vec![
        ("example.com", "user@example.com"),
        ("socialnetwork.com", "cooluser123"),
        ("bank.com", "secureuser"),
    ];

    if passwords.is_empty() {
        println!("{}", "No passwords saved yet.".yellow());
        return;
    }

    let selections: Vec<String> = passwords
        .iter()
        .map(|(site, username)| format!("{} ({})", site, username))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a password to view")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let (site, username) = passwords[selection];
    println!("\n{}", "Password details:".green());
    println!("Site: {}", site);
    println!("Username: {}", username);
    println!("Password: {}", "*".repeat(12));  // We don't actually show the password here

    println!("\n{}", "Options:".yellow());
    println!("1. Copy password to clipboard");
    println!("2. Show password (be careful!)");
    println!("3. Return to main menu");

    // Here you would implement the logic for these options
}



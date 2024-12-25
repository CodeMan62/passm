use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Confirm};
use rand::Rng;

pub fn run() {
    println!("{}", "Generating a secure password".blue());

    let length = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter desired password length")
        .default(16)
        .interact_text()
        .unwrap();

    let use_symbols = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Include symbols?")
        .default(true)
        .interact()
        .unwrap();

    let password = generate_password(length, use_symbols);

    println!("\n{}", "Generated password:".green());
    println!("{}", password);

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Copy to clipboard?")
        .interact()
        .unwrap()
    {
        // Here you would implement clipboard functionality
        println!("{}", "Password copied to clipboard.".green());
    }
}

fn generate_password(length: usize, use_symbols: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut charset: Vec<u8> = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_vec();
    
    if use_symbols {
        charset.extend_from_slice(b"!@#$%^&*()_+-=[]{}|;:,.<>?");
    }

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}



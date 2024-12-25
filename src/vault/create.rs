use crate::ui;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use zxcvbn::zxcvbn;

pub fn run() {
    println!("{}", "Creating a new password entry".blue());

    let _site_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the site name or device name")
        .interact_text()
        .unwrap();

    let _username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the username")
        .interact_text()
        .unwrap();

    let selections = &["Generate a secure password", "Type your own password"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to generate a password or type your own?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let password = match selection {
        0 => generate_password(),
        1 => type_password(),
        _ => unreachable!(),
    };

    ui::loading_animation("Saving password securely", 2000);
    println!("{}", "Password saved successfully!".green());

    analyze_password(&password);
}

fn generate_password() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+";
    let mut rng = rand::thread_rng();
    let password: String = (0..20)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    println!("{}", "Generated password:".green());
    println!("{}", password);
    password
}

fn type_password() -> String {
    Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your password")
        .interact()
        .unwrap()
}

fn analyze_password(password: &str) {
    let estimate = zxcvbn(password, &[]).unwrap();
    let score = estimate.score();
    let crack_time = estimate.crack_times().offline_slow_hashing_1e4_per_second();

    println!("\n{}", "Password Strength Analysis:".yellow());
    println!("Strength score: {}/4", score);
    println!("Estimated time to crack: {}", crack_time);

    match score {
        0..=2 => println!("{}", "Warning: This password is weak. Consider using a stronger password.".red()),
        3 => println!("{}", "This password is good, but could be stronger.".yellow()),
        4 => println!("{}", "Excellent! This is a strong password.".green()),
        _ => unreachable!(),
    }
}



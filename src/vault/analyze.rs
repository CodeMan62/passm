use colored::*;
use dialoguer::{theme::ColorfulTheme, Password};
use zxcvbn::zxcvbn;
use anyhow::Result;

pub fn run() -> Result<()> {
    println!("{}", "Analyze password strength".blue());

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a password to analyze")
        .interact()?;

    let estimate = zxcvbn(&password, &[]).unwrap();
    let score = estimate.score();
    let crack_time = estimate.crack_times().offline_slow_hashing_1e4_per_second();

    println!("\n{}", "Password Strength Analysis:".yellow());
    println!("Strength score: {}/4", score);
    println!("Estimated time to crack: {}", crack_time);

    match score {
        0..=1 => println!("{}", "This password is very weak and easily guessable.".red()),
        2 => println!("{}", "This password is weak and should not be used.".red()),
        3 => println!("{}", "This password is good, but could be stronger.".yellow()),
        4 => println!("{}", "Excellent! This is a strong password.".green()),
        _ => unreachable!(),
    }

    if let Some(feedback) = estimate.feedback() {
        if let Some(warning) = feedback.warning() {
            println!("\n{}", "Warning:".red());
            println!("{}", warning);
        }

        if !feedback.suggestions().is_empty() {
            println!("\n{}", "Suggestions to improve:".yellow());
            for suggestion in feedback.suggestions() {
                println!("- {}", suggestion);
            }
        }
    }

    Ok(())
}



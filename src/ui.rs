use colored::*;
use console::Term;
use std::{thread, time::Duration};

pub const BANNER: &str = r#"
 __      __   _    _  _   _____
 \ \    / /  /_\  | || | |_   _|
  \ \/\/ /  / _ \ | || |   | |
   \_/\_/  /_/ \_\|_||_|   |_|
"#;

pub fn display_menu(term: &Term) {
    term.clear_screen().unwrap();
    println!("{}", BANNER.bright_blue());
    println!("{}", "[ Vault - Your Secure Password Sanctuary ]".bright_green());
    println!("\nChoose an option:");
    println!("1. {} Create a new password", "🔒".green());
    println!("2. {} View passwords", "👁️ ".blue());
    println!("3. {} Generate a secure password", "🎲".magenta());
    println!("4. {} Analyze password strength", "💪".yellow());
    println!("Q. {} Exit", "🚪".red());
    print!("\n> ");
}

pub fn display_exit_animation() {
    print!("\n");
    for _ in 0..3 {
        print!("{}   \r", "Securing vault...".yellow());
        thread::sleep(Duration::from_millis(500));
        print!("{}     \r", "Securing vault...".bright_yellow());
        thread::sleep(Duration::from_millis(500));
    }
    println!("{}", "Vault secured. Goodbye!".green());
}

pub fn loading_animation(message: &str, duration_ms: u64) {
    let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let start_time = std::time::Instant::now();
    let duration = Duration::from_millis(duration_ms);

    while start_time.elapsed() < duration {
        for frame in spinner.iter() {
            print!("\r{} {} ", frame.cyan(), message);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
    print!("\r{} {} ", "✓".green(), message);
    println!();
}



mod cli;
mod encryption;
mod storage;
mod ui;


use cli::Cli;
use clap::Parser;
use colored::*;
use storage::Storage;

fn main() {
    print_welcome_screen();
    
    if let Err(e) = run() {
        eprintln!("{} {}", "ERROR:".bright_red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let ui = ui::UI::new();

    let master_password = ui.get_password("Enter master password")?;
    let storage = Storage::new(&master_password)?;

    cli.execute(storage, &ui)?;
    Ok(())
}

fn print_welcome_screen() {
    println!("\n{}", include_str!("../assets/banner.txt").bright_purple());
    println!("{}", "🔒 Secure Password Manager v0.1.0".bright_green().bold());
    println!("{}", "=====================================".bright_green());
    println!("{}", "Made with ❤️  by Your Name\n".bright_blue());
}


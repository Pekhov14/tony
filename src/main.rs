use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader};
use std::process::Command;
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "tony", about = "A simple task runner", version)]
struct Cli {
    command: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let file = File::open("tonyfile.json")?;
    let reader = BufReader::new(file);
    let commands: HashMap<String, String> = serde_json::from_reader(reader)?;

    let command_to_run = match commands.get(&cli.command) {
        Some(cmd) => cmd,
        None => {
            eprintln!("Command not found: '{}'", cli.command);
            std::process::exit(1);
        }
    };

    println!("{} {}", ">".green(), command_to_run.blue().bold());

    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    Command::new(shell)
        .arg(flag)
        .arg(command_to_run)
        .status()?;

    Ok(())
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader};
use std::process::Command;
use colored::Colorize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: tony <command>");
            std::process::exit(1);
        }
    };

    let file = File::open("tonyfile.json")?;
    let reader = BufReader::new(file);
    let commands: HashMap<String, String> = serde_json::from_reader(reader)?;

    let command_to_run = match commands.get(&command) {
        Some(cmd) => cmd,
        None => {
            eprintln!("Command not found: '{}'", command);
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

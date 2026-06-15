use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader};
use std::process::Command;
use colored::Colorize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        // TODO: create help output and version
        eprintln!("Usage: {} <command>", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];

    let file = File::open("tonyfile.json")?;
    let reader = BufReader::new(file);

    let commands: HashMap<String, String> = serde_json::from_reader(reader)?;

    if !commands.contains_key(command) {
        eprintln!("Command not found: '{}'", command);
        std::process::exit(1);
    }

    println!("{} {}", ">".green(), commands[command].blue().bold());
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    Command::new(shell)
        .arg(flag)
        .arg(commands[command].clone())
        .status()?;

    Ok(())
}

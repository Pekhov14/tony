use std::collections::HashMap;
use std::process::Command;
use clap::Parser;
use colored::Colorize;
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(name = "tony", about = "A simple task runner", version)]
struct Cli {
    command: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let file = std::fs::read_to_string("tonyfile.toml")
        .context("Failed to read 'tonyfile.toml'")?;

    let commands: HashMap<String, String> = toml::from_str(&file)
        .context("Invalid TOML format in 'tonyfile.toml'")?;

    let command_to_run = commands
        .get(&cli.command)
        .ok_or_else(|| anyhow::anyhow!("Command not found: '{}'", cli.command))?;

    println!("{} {}", ">".green(), command_to_run.blue().bold());

    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let status = Command::new(shell)
        .arg(flag)
        .arg(command_to_run)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .context("Failed to execute command")?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

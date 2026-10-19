use std::collections::HashMap;
use std::process::Command;
use clap::Parser;
use colored::Colorize;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "tony", about = "A simple task runner", version)]
struct Cli {
    command: String,
}
#[derive(Debug, Deserialize)]
struct Tonyfile {
    tasks: HashMap<String, Task>,
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Task {
    Simple(String),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let file_context = std::fs::read_to_string("tonyfile.toml")
        .context("Failed to read 'tonyfile.toml'")?;

    let tonyfile: Tonyfile = toml::from_str(&file_context)
        .context("Invalid TOML format in 'tonyfile.toml'")?;

    let task = tonyfile.tasks
        .get(&cli.command)
        .ok_or_else(|| anyhow::anyhow!("Command not found '{}'", cli.command))?;

    let command_to_run = match task {
        Task::Simple(cmd) => {
            cmd
        }
    };

    println!("{} {}", ">".green(), command_to_run.blue().bold() );

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

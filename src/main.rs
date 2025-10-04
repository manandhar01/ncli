use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::Path;

mod entityhash;

#[derive(Parser)]
#[command(name = "ncli")]
#[command(about = "NestJS helper CLI written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    EntityHash,
}

fn is_nest_project_root(path: &Path) -> bool {
    let indicators = ["nest-cli.json", "src/main.ts", "package.json"];
    indicators.iter().all(|file| path.join(file).exists())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cwd = std::env::current_dir()?;

    if !is_nest_project_root(&cwd) {
        eprintln!(
            "\n{}\n{}\n",
            "✖ Not a NestJS project root.".red().bold(),
            "Make sure you run this from your NestJS project directory.".yellow()
        );
        std::process::exit(1);
    }

    match cli.command {
        Commands::EntityHash => entityhash::run()?,
    }

    Ok(())
}

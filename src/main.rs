use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::{path::Path, process::Command};

mod entity_hash;
mod job_hash;
mod sync_module_components;

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
    JobHash,
    SyncModuleComponents,
    All,
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
        Commands::EntityHash => entity_hash::run()?,
        Commands::JobHash => job_hash::run()?,
        Commands::SyncModuleComponents => sync_module_components::run()?,
        Commands::All => {
            entity_hash::run()?;
            job_hash::run()?;
            sync_module_components::run()?;
        }
    }

    println!("{}", "→ Running formatter (npm run format)...".cyan());
    let status = Command::new("npm").arg("run").arg("format").status();

    match status {
        Ok(s) if s.success() => {
            println!(
                "\n{}\n",
                "✔ Formatting completed successfully.".green().bold()
            );
        }
        Ok(s) => {
            eprintln!(
                "\n{} (exit code: {})\n",
                "✖ Formatting failed.".red().bold(),
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!(
                "\n{} {}\n",
                "✖ Failed to run npm formatter:".red().bold(),
                e
            );
        }
    }

    Ok(())
}

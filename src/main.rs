use anyhow::Result;
use clap::Parser;
use colored::*;
use std::{env, path::Path};

mod cli;
mod commands;
mod generator;
mod sync;

use crate::cli::{Cli, MainCommand};
use crate::commands::{generate, sync as sync_command};

fn is_nest_project_root(path: &Path) -> bool {
    let indicators = ["nest-cli.json", "src/main.ts", "package.json"];
    indicators.iter().all(|file| path.join(file).exists())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cwd = env::current_dir()?;

    if !is_nest_project_root(&cwd) {
        eprintln!(
            "\n{}\n{}\n",
            "✖ Not a NestJS project root.".red().bold(),
            "Make sure you run this from your NestJS project directory.".yellow()
        );
        std::process::exit(1);
    }

    match cli.command {
        MainCommand::Sync(sync_cmd) => sync_command::handle(sync_cmd)?,
        MainCommand::Generate(gen_cmd) => generate::handle(gen_cmd)?,
    }

    commands::format::run_formatter();

    Ok(())
}


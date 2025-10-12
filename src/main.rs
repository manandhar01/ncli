use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::{path::Path, process::Command};

use crate::{
    generator::{
        controller, dto, entity, job, listing, module, processor, service, subscriber,
        utility::prompt,
    },
    sync::{entity_hash, job_hash, module_components},
};

mod generator;
mod sync;

#[derive(Parser)]
#[command(name = "ncli")]
#[command(about = "NestJS helper CLI written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: MainCommand,
}

#[derive(Subcommand)]
enum MainCommand {
    Sync(SyncCommand),
    Generate(GenerateCommand),
}

#[derive(Parser)]
struct SyncCommand {
    #[arg(value_enum, default_value_t = SyncTarget::All)]
    target: SyncTarget,
}

#[derive(clap::ValueEnum, Clone)]
enum SyncTarget {
    Entities,
    Jobs,
    Components,
    All,
}

#[derive(Parser)]
struct GenerateCommand {
    #[arg(value_enum, default_value_t = GenerateTarget::All)]
    target: GenerateTarget,
}

#[derive(clap::ValueEnum, Clone)]
enum GenerateTarget {
    Controller,
    Dto,
    Entity,
    Job,
    Listing,
    Module,
    Processor,
    Service,
    Subscriber,
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
        MainCommand::Sync(sync) => match sync.target {
            SyncTarget::Entities => entity_hash::run()?,
            SyncTarget::Jobs => job_hash::run()?,
            SyncTarget::Components => module_components::run()?,
            SyncTarget::All => {
                entity_hash::run()?;
                job_hash::run()?;
                module_components::run()?;
            }
        },
        MainCommand::Generate(generate) => match generate.target {
            GenerateTarget::Controller => controller::run(None, None, None)?,
            GenerateTarget::Dto => dto::run(None, None, None)?,
            GenerateTarget::Entity => entity::run(None, None, None)?,
            GenerateTarget::Job => job::run(None, None, None)?,
            GenerateTarget::Listing => listing::run(None, None, None)?,
            GenerateTarget::Module => module::run(None)?,
            GenerateTarget::Processor => processor::run(None, None, None)?,
            GenerateTarget::Service => service::run(None, None, None)?,
            GenerateTarget::Subscriber => subscriber::run(None, None, None)?,
            GenerateTarget::All => {
                let table_name = prompt("Enter table name:")?;
                let entity_name = prompt("Enter entity name:")?;
                let module_name = prompt("Enter module name:")?;

                module::run(Some(&module_name))?;
                controller::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;
                dto::run(Some(&entity_name), Some(&module_name), Some("list.filter"))?;
                dto::run(Some(&entity_name), Some(&module_name), Some("find"))?;
                dto::run(Some(&entity_name), Some(&module_name), Some("add"))?;
                entity::run(Some(&table_name), Some(&entity_name), Some(&module_name))?;
                job::run(Some(&entity_name), Some(&module_name), Some(&entity_name))?;
                listing::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;
                processor::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;
                service::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;
                subscriber::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;
            }
        },
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

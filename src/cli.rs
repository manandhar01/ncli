use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "ncli")]
#[command(about = "NestJS helper CLI written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: MainCommand,
}

#[derive(Subcommand)]
pub enum MainCommand {
    Sync(SyncCommand),
    Generate(GenerateCommand),
}

#[derive(Parser)]
pub struct SyncCommand {
    #[arg(value_enum, default_value_t = SyncTarget::All)]
    pub target: SyncTarget,
}

#[derive(ValueEnum, Clone)]
pub enum SyncTarget {
    Entities,
    Jobs,
    Components,
    All,
}

#[derive(Parser)]
pub struct GenerateCommand {
    #[arg(value_enum, default_value_t = GenerateTarget::All)]
    pub target: GenerateTarget,
}

#[derive(ValueEnum, Clone)]
pub enum GenerateTarget {
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

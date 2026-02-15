use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "ncli",
    version,
    about = "A helper CLI for managing and generating NestJS project resources.",
    long_about = "ncli is a Rust-powered CLI tool that helps you sync and generate resources inside a NestJS project (entities, services, modules, jobs, etc.)."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: MainCommand,
}

#[derive(Subcommand)]
pub enum MainCommand {
    #[command(
        about = "Sync existing NestJS resources",
        long_about = "Sync project resources such as entities, jobs, or components to ensure the project structure stays consistent."
    )]
    Sync(SyncCommand),

    #[command(
        about = "Generate new NestJS resources",
        long_about = "Generate new NestJS building blocks like controllers, DTOs, entities, services, modules, jobs, and more."
    )]
    Generate(GenerateCommand),
}

#[derive(Parser)]
#[command(
    about = "Specify what type of resources to sync",
    long_about = "Choose which resource type should be synchronized. Defaults to 'all' if not specified."
)]
pub struct SyncCommand {
    #[arg(
        value_enum,
        default_value_t = SyncTarget::All,
        help = "Target resource type to sync"
    )]
    pub target: SyncTarget,
}

#[derive(ValueEnum, Clone)]
pub enum SyncTarget {
    #[value(help = "Sync entity files")]
    Entities,

    #[value(help = "Sync job/queue related files")]
    Jobs,

    #[value(help = "Sync module components")]
    Components,

    #[value(help = "Sync column mappings for entities")]
    ColumnMappings,

    #[value(help = "Sync all supported resource types")]
    All,
}

#[derive(Parser)]
#[command(
    about = "Specify what type of resources to generate",
    long_about = "Choose which NestJS resource should be generated. Defaults to 'all' if not specified."
)]
pub struct GenerateCommand {
    #[arg(
        value_enum,
        default_value_t = GenerateTarget::All,
        help = "Target resource type to generate"
    )]
    pub target: GenerateTarget,
}

#[derive(ValueEnum, Clone)]
pub enum GenerateTarget {
    #[value(help = "Generate a controller")]
    Controller,

    #[value(help = "Generate a DTO (Data Transfer Object)")]
    Dto,

    #[value(help = "Generate an entity")]
    Entity,

    #[value(help = "Generate a background job")]
    Job,

    #[value(help = "Generate a listing structure")]
    Listing,

    #[value(help = "Generate a module")]
    Module,

    #[value(help = "Generate a data processor")]
    Processor,

    #[value(help = "Generate a service")]
    Service,

    #[value(help = "Generate an event subscriber")]
    Subscriber,

    #[value(help = "Generate all supported resource types")]
    All,
}

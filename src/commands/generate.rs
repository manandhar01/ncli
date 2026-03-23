use core::option::Option::Some;

use crate::cli::{GenerateCommand, GenerateTarget};
use crate::generator::{
    controller, dto, entity, job, listing, module, processor, service, subscriber,
    utility::{prompt, DtoSpec, EntitySpec},
};
use anyhow::Result;
use colored::*;

pub fn handle(cmd: GenerateCommand) -> Result<()> {
    match cmd.target {
        GenerateTarget::Controller => {
            controller::run(None, None, EntitySpec { name: None, entity_type: None })?
        }
        GenerateTarget::Dto => dto::run(None, None, DtoSpec { dto_type: None, entity_type: None })?,
        GenerateTarget::Entity => entity::run(None, None, None)?,
        GenerateTarget::Job => job::run(None, None, None)?,
        GenerateTarget::Listing => {
            listing::run(None, None, EntitySpec { name: None, entity_type: None })?
        }
        GenerateTarget::Module => module::run(None)?,
        GenerateTarget::Processor => {
            processor::run(None, None, EntitySpec { name: None, entity_type: None })?
        }
        GenerateTarget::Service => {
            service::run(None, None, EntitySpec { name: None, entity_type: None })?
        }
        GenerateTarget::Subscriber => subscriber::run(None, None, None)?,
        GenerateTarget::All => generate_all()?,
    }
    Ok(())
}

struct AllContext {
    table_name: String,
    entity_name: String,
    module_name: String,
}

struct ControllerOptions {
    generate_listing: bool,
    generate_processor: bool,
    entity_type: String,
}

fn prompt_required(msg: &str, field: &str) -> Result<String> {
    let value = prompt(msg)?;
    if value.trim().is_empty() {
        eprintln!(
            "\n{}\n",
            format!("✖ Error: {field} is a mandatory field.").red().bold()
        );
        std::process::exit(1);
    }
    Ok(value)
}

fn collect_all_context() -> Result<AllContext> {
    Ok(AllContext {
        table_name: prompt_required("Enter table name:", "Table name")?,
        entity_name: prompt_required("Enter entity name:", "Entity name")?,
        module_name: prompt_required("Enter module name:", "Module name")?,
    })
}

fn prompt_yes_no(msg: &str) -> Result<bool> {
    let input = prompt(msg)?;
    Ok(input.trim().is_empty() || input.trim().eq_ignore_ascii_case("y"))
}

fn collect_controller_options() -> Result<ControllerOptions> {
    let entity_type = {
        let input = prompt(
            "Select entity type to generate:\n1) Business specific\n2) User specific\n3) Both (default)\nEnter choice (1/2/3):",
        )?;
        match input.trim() {
            "1" => "business".to_string(),
            "2" => "user".to_string(),
            _ => "both".to_string(),
        }
    };
    Ok(ControllerOptions {
        generate_listing: prompt_yes_no("Generate listing? (Y/n):")?,
        generate_processor: prompt_yes_no("Generate processor? (Y/n):")?,
        entity_type,
    })
}

fn generate_core(ctx: &AllContext) -> Result<()> {
    module::run(Some(&ctx.module_name))?;
    entity::run(Some(&ctx.table_name), Some(&ctx.entity_name), Some(&ctx.module_name))?;
    job::run(Some(&ctx.entity_name), Some(&ctx.module_name), Some(&ctx.entity_name))?;
    subscriber::run(Some(&ctx.entity_name), Some(&ctx.entity_name), Some(&ctx.module_name))?;
    Ok(())
}

enum ChildResource {
    Listing,
    Processor,
}

fn generate_child_resource(ctx: &AllContext, resource: ChildResource, entity_type: &str) -> Result<()> {
    type RunFn = fn(Option<&str>, Option<&str>, EntitySpec) -> Result<()>;
    let (dto_type, run): (&str, RunFn) = match resource {
        ChildResource::Listing => ("list.filter", listing::run),
        ChildResource::Processor => ("add", processor::run),
    };
    dto::run(
        Some(&ctx.entity_name),
        Some(&ctx.module_name),
        DtoSpec { dto_type: Some(dto_type.to_string()), entity_type: Some(entity_type.to_string()) },
    )?;
    run(
        Some(&ctx.entity_name),
        Some(&ctx.module_name),
        EntitySpec { name: Some(ctx.entity_name.clone()), entity_type: Some(entity_type.to_string()) },
    )
}

fn generate_controller_resources(ctx: &AllContext, opts: &ControllerOptions) -> Result<()> {
    controller::run(
        Some(&ctx.entity_name),
        Some(&ctx.module_name),
        EntitySpec {
            name: Some(ctx.entity_name.clone()),
            entity_type: Some(opts.entity_type.clone()),
        },
    )?;
    dto::run(
        Some(&ctx.entity_name),
        Some(&ctx.module_name),
        DtoSpec {
            dto_type: Some("find".to_string()),
            entity_type: Some(opts.entity_type.clone()),
        },
    )?;
    service::run(
        Some(&ctx.entity_name),
        Some(&ctx.module_name),
        EntitySpec {
            name: Some(ctx.entity_name.clone()),
            entity_type: Some(opts.entity_type.clone()),
        },
    )?;
    if opts.generate_listing {
        generate_child_resource(ctx, ChildResource::Listing, &opts.entity_type)?;
    }
    if opts.generate_processor {
        generate_child_resource(ctx, ChildResource::Processor, &opts.entity_type)?;
    }
    Ok(())
}

fn generate_all() -> Result<()> {
    let ctx = collect_all_context()?;

    let generate_controller = prompt_yes_no("Generate controller? (Y/n):")?;
    let opts = if generate_controller { Some(collect_controller_options()?) } else { None };

    generate_core(&ctx)?;

    if let Some(opts) = opts {
        generate_controller_resources(&ctx, &opts)?;
    }

    Ok(())
}

use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt, EntitySpec},
};

const PROCESSOR_STUB: &str = include_str!("../../templates/process.data.stub");
const BUSINESS_PROCESSOR_STUB: &str = include_str!("../../templates/business.process.data.stub");
const USER_PROCESSOR_STUB: &str = include_str!("../../templates/user.process.data.stub");

pub fn run(
    processor_name: Option<&str>,
    module_name: Option<&str>,
    entity_spec: EntitySpec,
) -> Result<()> {
    let processor_name = match processor_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter processor name:")?,
    };
    let entity_name = match entity_spec.name {
        Some(ref name) if !name.is_empty() => name.clone(),
        _ => prompt("Enter entity name:")?,
    };
    let module_name = match module_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter module name:")?,
    };
    let entity_type = match entity_spec.entity_type {
        Some(ref t) if !t.is_empty() => t.clone(),
        _ => {
            let input = prompt(
                "Select entity type to generate:\n1) Business specific\n2) User specific\n3) Both (default)\nEnter choice (1/2/3):",
            )?;
            match input.trim() {
                "1" => "business".to_string(),
                "2" => "user".to_string(),
                _ => "both".to_string(),
            }
        }
    };

    module::run(Some(&module_name))?;

    let cwd =
        std::env::current_dir().with_context(|| "Failed to get current working directory".red())?;
    let src_path = cwd.join("src");

    let types = if entity_type == "both" {
        vec!["business.", "user."]
    } else {
        vec![""]
    };

    for t in types {
        generate_processor(&processor_name, t, &ProcessorContext { entity_name: &entity_name, module_name: &module_name, src_path: &src_path })?;
    }

    Ok(())
}

struct ProcessorContext<'a> {
    entity_name: &'a str,
    module_name: &'a str,
    src_path: &'a std::path::Path,
}

fn generate_processor(name: &str, entity_type: &str, ctx: &ProcessorContext<'_>) -> Result<()> {
    let processor_file_name = get_file_name(name);
    let processor_dir = ctx.src_path.join(ctx.module_name).join("libraries");
    let processor_file = processor_dir.join(format!(
        "process.{entity_type}{processor_file_name}.data.ts"
    ));

    if processor_file.exists() {
        return Ok(());
    }

    fs::create_dir_all(&processor_dir).with_context(|| {
        format!(
            "Failed to create processor directory at {}",
            get_relative_path(&processor_dir)
        )
        .red()
    })?;

    let template = match entity_type {
        "business." => BUSINESS_PROCESSOR_STUB,
        "user." => USER_PROCESSOR_STUB,
        _ => PROCESSOR_STUB,
    };

    let content = template
        .replace("{{ProcessorName}}", name)
        .replace("{{EntityName}}", ctx.entity_name);

    fs::write(&processor_file, content).with_context(|| {
        format!(
            "Failed to write processor file at {}",
            get_relative_path(&processor_file)
        )
        .red()
    })?;

    println!(
        "{} {}",
        "✔ Processor created:".green(),
        get_relative_path(&processor_file)
    );

    Ok(())
}

use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt, EntitySpec},
};

const PROCESSOR_STUB: &str = include_str!("../../templates/process.data.stub");

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

    module::run(Some(&module_name))?;

    let cwd =
        std::env::current_dir().with_context(|| "Failed to get current working directory".red())?;
    let src_path = cwd.join("src");

    generate_processor(
        &processor_name,
        &ProcessorContext { entity_name: &entity_name, module_name: &module_name, src_path: &src_path },
    )?;

    Ok(())
}

struct ProcessorContext<'a> {
    entity_name: &'a str,
    module_name: &'a str,
    src_path: &'a std::path::Path,
}

fn generate_processor(name: &str, ctx: &ProcessorContext<'_>) -> Result<()> {
    let processor_file_name = get_file_name(name);
    let entity_file_name = get_file_name(ctx.entity_name);
    let processor_dir = ctx.src_path.join(ctx.module_name).join("libraries");
    let processor_file = processor_dir.join(format!("process.{processor_file_name}.data.ts"));

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

    let content = PROCESSOR_STUB
        .replace("{{ProcessorName}}", name)
        .replace("{{EntityName}}", ctx.entity_name)
        .replace("{{processor_file}}", &processor_file_name)
        .replace("{{entity_file}}", &entity_file_name);

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

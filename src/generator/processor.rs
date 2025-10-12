use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt},
};

const PROCESSOR_STUB: &str = include_str!("../../templates/process.data.stub");

pub fn run(
    processor_name: Option<&str>,
    entity_name: Option<&str>,
    module_name: Option<&str>,
) -> Result<()> {
    let processor_name = match processor_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter processor name:")?,
    };
    let entity_name = match entity_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter entity name:")?,
    };
    let module_name = match module_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter module name:")?,
    };

    let cwd =
        std::env::current_dir().with_context(|| "Failed to get current working directory".red())?;
    let src_path = cwd.join("src");

    module::run(Some(&module_name))?;

    let processor_file_name = get_file_name(&processor_name);
    let processor_dir = src_path.join(&module_name).join("libraries");
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
        .replace("{{ProcessorName}}", &processor_name)
        .replace("{{EntityName}}", &entity_name);

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

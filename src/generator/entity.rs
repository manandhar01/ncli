use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt},
};

const ENTITY_STUB: &str = include_str!("../../templates/entity.stub");

pub fn run(
    table_name: Option<&str>,
    entity_name: Option<&str>,
    module_name: Option<&str>,
) -> Result<()> {
    let table_name = match table_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter table name:")?,
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

    let entity_file_name = get_file_name(&entity_name);
    let entity_dir = src_path.join(&module_name).join("entities");
    let entity_file = entity_dir.join(format!("{entity_file_name}.entity.ts"));

    if entity_file.exists() {
        return Ok(());
    }

    fs::create_dir_all(&entity_dir).with_context(|| {
        format!(
            "Failed to create entity directory at {}",
            get_relative_path(&entity_dir)
        )
        .red()
    })?;

    let content = ENTITY_STUB
        .replace("{{TableName}}", &table_name)
        .replace("{{EntityName}}", &entity_name);

    fs::write(&entity_file, content).with_context(|| {
        format!(
            "Failed to write entity file at {}",
            get_relative_path(&entity_file)
        )
        .red()
    })?;

    println!(
        "{} {}",
        "✔ Entity created:".green(),
        get_relative_path(&entity_file)
    );

    Ok(())
}

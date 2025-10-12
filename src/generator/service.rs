use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt},
};

const SERVICE_STUB: &str = include_str!("../../templates/service.stub");

pub fn run(
    service_name: Option<&str>,
    entity_name: Option<&str>,
    module_name: Option<&str>,
) -> Result<()> {
    let service_name = match service_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter service name:")?,
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

    let service_file_name = get_file_name(&service_name);
    let service_dir = src_path.join(&module_name).join("services");
    let service_file = service_dir.join(format!("{service_file_name}.service.ts"));

    if service_file.exists() {
        return Ok(());
    }

    fs::create_dir_all(&service_dir).with_context(|| {
        format!(
            "Failed to create service directory at {}",
            get_relative_path(&service_dir)
        )
        .red()
    })?;

    let content = SERVICE_STUB
        .replace("{{ServiceName}}", &service_name)
        .replace("{{EntityName}}", &entity_name);

    fs::write(&service_file, content).with_context(|| {
        format!(
            "Failed to write service file at {}",
            get_relative_path(&service_file)
        )
        .red()
    })?;

    println!(
        "{} {}",
        "✔ Service created:".green(),
        get_relative_path(&service_file)
    );

    Ok(())
}

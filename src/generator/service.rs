use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt},
};

const SERVICE_STUB: &str = include_str!("../../templates/service.stub");
const BUSINESS_SERVICE_STUB: &str = include_str!("../../templates/business.service.stub");
const USER_SERVICE_STUB: &str = include_str!("../../templates/user.service.stub");

pub fn run(
    service_name: Option<&str>,
    entity_name: Option<&str>,
    module_name: Option<&str>,
    entity_type: Option<&str>,
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
    let entity_type = match entity_type {
        Some(t) if !t.is_empty() => t.to_string(),
        _ => {
            let input = prompt(
                "Select entity type to generate:\n1) Business specific\n2) User specific\n3) Both (default)\nEnter choice (1/2/3):",
            )?;

            let entity_type = match input.trim() {
                "1" => "business",
                "2" => "user",
                "3" => "both",
                _ => "both",
            };
            entity_type.to_string()
        }
    };

    let cwd =
        std::env::current_dir().with_context(|| "Failed to get current working directory".red())?;
    let src_path = cwd.join("src");

    module::run(Some(&module_name))?;

    let types = if entity_type == "both" {
        vec!["business.", "user."]
    } else {
        vec![""]
    };

    for t in types {
        generate_service(&service_name, &entity_name, &module_name, t, &src_path)?;
    }

    Ok(())
}

fn generate_service(
    service_name: &str,
    entity_name: &str,
    module_name: &str,
    entity_type: &str,
    src_path: &std::path::Path,
) -> Result<()> {
    let service_file_name = get_file_name(service_name);
    let service_dir = src_path.join(module_name).join("services");
    let service_file = service_dir.join(format!("{entity_type}{service_file_name}.service.ts"));

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

    let stub = match entity_type {
        "business." => BUSINESS_SERVICE_STUB,
        "user." => USER_SERVICE_STUB,
        _ => SERVICE_STUB,
    };

    let content = stub
        .replace("{{ServiceName}}", service_name)
        .replace("{{EntityName}}", entity_name);

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

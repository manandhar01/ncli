use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt, to_camel_case},
};

const BUSINESS_CONTROLLER_STUB: &str = include_str!("../../templates/business.controller.stub");
const USER_CONTROLLER_STUB: &str = include_str!("../../templates/user.controller.stub");
const CONTROLLER_STUB: &str = include_str!("../../templates/controller.stub");

pub fn run(
    controller_name: Option<&str>,
    entity_name: Option<&str>,
    module_name: Option<&str>,
    entity_type: Option<&str>,
) -> Result<()> {
    let controller_name = match controller_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter controller name:")?,
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
        generate_controller(&controller_name, &entity_name, &module_name, t, &src_path)?;
    }

    Ok(())
}

fn generate_controller(
    controller_name: &str,
    entity_name: &str,
    module_name: &str,
    entity_type: &str,
    src_path: &std::path::Path,
) -> Result<()> {
    let controller_file_name = get_file_name(controller_name);
    let controller_dir = src_path.join(module_name).join("controllers");
    let controller_file =
        controller_dir.join(format!("{entity_type}{controller_file_name}.controller.ts"));

    if controller_file.exists() {
        return Ok(());
    }

    fs::create_dir_all(&controller_dir).with_context(|| {
        format!(
            "Failed to create controller directory at {}",
            get_relative_path(&controller_dir)
        )
        .red()
    })?;

    let controller_var = to_camel_case(controller_name);

    let template = match entity_type {
        "business." => BUSINESS_CONTROLLER_STUB,
        "user." => USER_CONTROLLER_STUB,
        _ => CONTROLLER_STUB,
    };

    let content = template
        .replace("{{ControllerName}}", controller_name)
        .replace("{{controller_var}}", &controller_var)
        .replace("{{EntityName}}", entity_name);

    fs::write(&controller_file, content).with_context(|| {
        format!(
            "Failed to write controller file at {}",
            get_relative_path(&controller_file)
        )
        .red()
    })?;

    println!(
        "{} {}",
        "✔ Controller created:".green(),
        get_relative_path(&controller_file)
    );

    Ok(())
}

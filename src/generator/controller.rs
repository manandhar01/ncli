use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt, to_camel_case, EntitySpec},
};

const BUSINESS_CONTROLLER_STUB: &str = include_str!("../../templates/business.controller.stub");
const USER_CONTROLLER_STUB: &str = include_str!("../../templates/user.controller.stub");
const CONTROLLER_STUB: &str = include_str!("../../templates/controller.stub");

pub fn run(
    controller_name: Option<&str>,
    module_name: Option<&str>,
    entity_spec: EntitySpec,
) -> Result<()> {
    let controller_name = match controller_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter controller name:")?,
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
        generate_controller(&controller_name, t, &ControllerContext { entity_name: &entity_name, module_name: &module_name, src_path: &src_path })?;
    }

    Ok(())
}

struct ControllerContext<'a> {
    entity_name: &'a str,
    module_name: &'a str,
    src_path: &'a std::path::Path,
}

fn generate_controller(name: &str, entity_type: &str, ctx: &ControllerContext<'_>) -> Result<()> {
    let controller_file_name = get_file_name(name);
    let controller_dir = ctx.src_path.join(ctx.module_name).join("controllers");
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

    let controller_var = to_camel_case(name);

    let template = match entity_type {
        "business." => BUSINESS_CONTROLLER_STUB,
        "user." => USER_CONTROLLER_STUB,
        _ => CONTROLLER_STUB,
    };

    let content = template
        .replace("{{ControllerName}}", name)
        .replace("{{controller_var}}", &controller_var)
        .replace("{{EntityName}}", ctx.entity_name);

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

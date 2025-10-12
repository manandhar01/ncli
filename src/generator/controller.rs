use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt, to_camel_case},
};

const CONTROLLER_STUB: &str = include_str!("../../templates/controller.stub");

pub fn run(
    controller_name: Option<&str>,
    entity_name: Option<&str>,
    module_name: Option<&str>,
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

    let cwd =
        std::env::current_dir().with_context(|| "Failed to get current working directory".red())?;
    let src_path = cwd.join("src");

    module::run(Some(&module_name))?;

    let controller_file_name = get_file_name(&controller_name);
    let controller_dir = src_path.join(&module_name).join("controllers");
    let controller_file = controller_dir.join(format!("{controller_file_name}.controller.ts"));

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

    let controller_var = to_camel_case(&controller_name);
    let content = CONTROLLER_STUB
        .replace("{{ControllerName}}", &controller_name)
        .replace("{{controller_var}}", &controller_var)
        .replace("{{EntityName}}", &entity_name);

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

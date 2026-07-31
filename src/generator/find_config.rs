use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt, to_camel_case},
};

const FIND_CONFIG_STUB: &str = include_str!("../../templates/find.config.stub");

pub fn run(
    config_name: Option<&str>,
    module_name: Option<&str>,
    entity_name: Option<&str>,
) -> Result<()> {
    let config_name = match config_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter find config name:")?,
    };
    let entity_name = match entity_name {
        Some(name) if !name.is_empty() => name.to_string(),
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

    generate_find_config(
        &config_name,
        &FindConfigContext { entity_name: &entity_name, module_name: &module_name, src_path: &src_path },
    )?;

    Ok(())
}

struct FindConfigContext<'a> {
    entity_name: &'a str,
    module_name: &'a str,
    src_path: &'a std::path::Path,
}

fn generate_find_config(name: &str, ctx: &FindConfigContext<'_>) -> Result<()> {
    let config_file_name = get_file_name(name);
    let entity_file_name = get_file_name(ctx.entity_name);
    let name_camel = to_camel_case(name);
    let config_dir = ctx.src_path.join(ctx.module_name).join("libraries");
    let config_file = config_dir.join(format!("{config_file_name}.find.config.ts"));

    if config_file.exists() {
        return Ok(());
    }

    fs::create_dir_all(&config_dir).with_context(|| {
        format!(
            "Failed to create find config directory at {}",
            get_relative_path(&config_dir)
        )
        .red()
    })?;

    let content = FIND_CONFIG_STUB
        .replace("{{EntityName}}", ctx.entity_name)
        .replace("{{entity_file}}", &entity_file_name)
        .replace("{{name_camel}}", &name_camel);

    fs::write(&config_file, content).with_context(|| {
        format!(
            "Failed to write find config file at {}",
            get_relative_path(&config_file)
        )
        .red()
    })?;

    println!(
        "{} {}",
        "✔ Find config created:".green(),
        get_relative_path(&config_file)
    );

    Ok(())
}

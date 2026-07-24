use anyhow::{Context, Result};
use colored::*;
use std::{fs, path::Path};

use crate::generator::utility::{capitalize_first, get_relative_path, prompt};

const MODULE_STUB: &str = include_str!("../../templates/module.stub");

pub fn run(module_name: Option<&str>) -> Result<()> {
    let module_name = match module_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter module name:")?,
    };

    let cwd =
        std::env::current_dir().with_context(|| "Failed to get current working directory".red())?;
    let src_path = cwd.join("src");
    let module_dir = src_path.join(&module_name);

    if !module_dir.exists() {
        fs::create_dir_all(&module_dir).with_context(|| {
            format!(
                "Failed to create module directory at {}",
                get_relative_path(&module_dir)
            )
            .red()
        })?;

        create_module_file(&src_path, &module_name)?;
    } else {
        let module_file = module_dir.join(format!("{module_name}.module.ts"));

        if !module_file.exists() {
            create_module_file(&src_path, &module_name)?;
        }
    }

    Ok(())
}

fn create_module_file(src_path: &Path, module_name: &str) -> Result<()> {
    let module_dir = src_path.join(module_name);
    let dest_path = module_dir.join(format!("{module_name}.module.ts"));
    let module_var = capitalize_first(module_name);

    let content = MODULE_STUB
        .replace("{{ModuleName}}", module_name)
        .replace("{{ModuleVar}}", &module_var);

    fs::write(&dest_path, content).with_context(|| {
        format!(
            "Failed to write module file at {}",
            get_relative_path(&dest_path)
        )
        .red()
    })?;

    println!(
        "{} {}",
        "✔ Created module file:".green(),
        get_relative_path(&dest_path)
    );

    Ok(())
}

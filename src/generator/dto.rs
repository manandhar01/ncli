use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt, DtoSpec},
};

const DTO_DEFAULT_STUB: &str = include_str!("../../templates/dto.stub");
const DTO_FIND_STUB: &str = include_str!("../../templates/find.dto.stub");
const DTO_LIST_FILTER_STUB: &str = include_str!("../../templates/list.filter.dto.stub");
const DTO_ADD_STUB: &str = include_str!("../../templates/add.dto.stub");

pub fn run(dto_name: Option<&str>, module_name: Option<&str>, dto_spec: DtoSpec) -> Result<()> {
    let dto_name = match dto_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter dto name:")?,
    };
    let module_name = match module_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter module name:")?,
    };
    let dto_type = match dto_spec.dto_type {
        Some(ref t) if !t.is_empty() => t.clone(),
        _ => {
            let input = prompt(
                "Select dto type to generate:\n1) Default (default)\n2) Find\n3) List filter\n4) Add\nEnter choice (1/2/3/4):",
            )?;
            match input.trim() {
                "1" => "default".to_string(),
                "2" => "find".to_string(),
                "3" => "list.filter".to_string(),
                "4" => "add".to_string(),
                _ => "default".to_string(),
            }
        }
    };

    module::run(Some(&module_name))?;

    let cwd =
        std::env::current_dir().with_context(|| "Failed to get current working directory".red())?;
    let src_path = cwd.join("src");

    generate_dto(
        &dto_name,
        &DtoContext { module_name: &module_name, dto_type: &dto_type, src_path: &src_path },
    )?;

    Ok(())
}

struct DtoContext<'a> {
    module_name: &'a str,
    dto_type: &'a str,
    src_path: &'a std::path::Path,
}

fn generate_dto(dto_name: &str, ctx: &DtoContext<'_>) -> Result<()> {
    let dto_file_name = get_file_name(dto_name);
    let dto_dir = ctx.src_path.join(ctx.module_name).join("dtos");
    let dto_file = match ctx.dto_type {
        "find" => dto_dir.join(format!("{dto_file_name}.find.dto.ts")),
        "list.filter" => dto_dir.join(format!("{dto_file_name}.list.filter.dto.ts")),
        "add" => dto_dir.join(format!("add.{dto_file_name}.dto.ts")),
        _ => dto_dir.join(format!("{dto_file_name}.dto.ts")),
    };

    if dto_file.exists() {
        return Ok(());
    }

    fs::create_dir_all(&dto_dir).with_context(|| {
        format!(
            "Failed to create dto directory at {}",
            get_relative_path(&dto_dir)
        )
        .red()
    })?;

    let template = match ctx.dto_type {
        "find" => DTO_FIND_STUB,
        "list.filter" => DTO_LIST_FILTER_STUB,
        "add" => DTO_ADD_STUB,
        _ => DTO_DEFAULT_STUB,
    };

    let content = template.replace("{{DtoName}}", dto_name);

    fs::write(&dto_file, content).with_context(|| {
        format!(
            "Failed to write dto file at {}",
            get_relative_path(&dto_file)
        )
    })?;

    println!(
        "{} {}",
        "✔ Dto created:".green(),
        get_relative_path(&dto_file)
    );

    Ok(())
}

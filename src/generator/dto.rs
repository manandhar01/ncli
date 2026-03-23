use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt, DtoSpec},
};

const DTO_DEFAULT_STUB: &str = include_str!("../../templates/dto.stub");

const DTO_FIND_STUB: &str = include_str!("../../templates/find.dto.stub");
const DTO_BUSINESS_FIND_STUB: &str = include_str!("../../templates/business.find.dto.stub");
const DTO_USER_FIND_STUB: &str = include_str!("../../templates/user.find.dto.stub");

const DTO_LIST_FILTER_STUB: &str = include_str!("../../templates/list.filter.dto.stub");
const DTO_BUSINESS_LIST_FILTER_STUB: &str =
    include_str!("../../templates/business.list.filter.dto.stub");
const DTO_USER_LIST_FILTER_STUB: &str = include_str!("../../templates/user.list.filter.dto.stub");

const DTO_ADD_STUB: &str = include_str!("../../templates/add.dto.stub");
const DTO_BUSINESS_ADD_STUB: &str = include_str!("../../templates/business.add.dto.stub");
const DTO_USER_ADD_STUB: &str = include_str!("../../templates/user.add.dto.stub");

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
    let entity_type = match dto_spec.entity_type {
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
        generate_dto(&dto_name, t, &DtoContext { module_name: &module_name, dto_type: &dto_type, src_path: &src_path })?;
    }

    Ok(())
}

struct DtoContext<'a> {
    module_name: &'a str,
    dto_type: &'a str,
    src_path: &'a std::path::Path,
}

fn generate_dto(dto_name: &str, entity_type: &str, ctx: &DtoContext<'_>) -> Result<()> {
    let dto_file_name = get_file_name(dto_name);
    let dto_dir = ctx.src_path.join(ctx.module_name).join("dtos");
    let dto_file = match ctx.dto_type {
        "find" => dto_dir.join(format!("{entity_type}{dto_file_name}.find.dto.ts")),
        "list.filter" => dto_dir.join(format!("{entity_type}{dto_file_name}.list.filter.dto.ts")),
        "add" => dto_dir.join(format!("add.{entity_type}{dto_file_name}.dto.ts")),
        _ => dto_dir.join(format!("{entity_type}{dto_file_name}.dto.ts")),
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

    let template = match (ctx.dto_type, entity_type) {
        ("find", "business.") => DTO_BUSINESS_FIND_STUB,
        ("find", "user.") => DTO_USER_FIND_STUB,
        ("find", _) => DTO_FIND_STUB,
        ("list.filter", "business.") => DTO_BUSINESS_LIST_FILTER_STUB,
        ("list.filter", "user.") => DTO_USER_LIST_FILTER_STUB,
        ("list.filter", _) => DTO_LIST_FILTER_STUB,
        ("add", "business.") => DTO_BUSINESS_ADD_STUB,
        ("add", "user.") => DTO_USER_ADD_STUB,
        ("add", _) => DTO_ADD_STUB,
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

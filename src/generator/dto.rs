use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt},
};

const DTO_DEFAULT_STUB: &str = include_str!("../../templates/dto.stub");
const DTO_FIND_STUB: &str = include_str!("../../templates/find.dto.stub");
const DTO_LIST_FILTER_STUB: &str = include_str!("../../templates/list.filter.dto.stub");
const DTO_ADD_STUB: &str = include_str!("../../templates/add.dto.stub");

pub fn run(
    dto_name: Option<&str>,
    module_name: Option<&str>,
    dto_type: Option<&str>,
) -> Result<()> {
    let dto_name = match dto_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter dto name:")?,
    };
    let module_name = match module_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter module name:")?,
    };

    let cwd =
        std::env::current_dir().with_context(|| "Failed to get current working directory".red())?;
    let src_path = cwd.join("src");

    module::run(Some(&module_name))?;

    let dto_file_name = get_file_name(&dto_name);
    let dto_dir = src_path.join(&module_name).join("dtos");

    let (dto_file, template) = match dto_type {
        Some("find") => (
            dto_dir.join(format!("{dto_file_name}.find.dto.ts")),
            DTO_FIND_STUB,
        ),
        Some("list.filter") => (
            dto_dir.join(format!("{dto_file_name}.list.filter.dto.ts")),
            DTO_LIST_FILTER_STUB,
        ),
        Some("add") => (
            dto_dir.join(format!("add.{dto_file_name}.dto.ts")),
            DTO_ADD_STUB,
        ),
        _ => (
            dto_dir.join(format!("{dto_file_name}.dto.ts")),
            DTO_DEFAULT_STUB,
        ),
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

    let content = template.replace("{{DtoName}}", &dto_name);

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

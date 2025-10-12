use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt},
};

const LISTING_STUB: &str = include_str!("../../templates/process.list.stub");

pub fn run(
    listing_name: Option<&str>,
    entity_name: Option<&str>,
    module_name: Option<&str>,
) -> Result<()> {
    let listing_name = match listing_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter listing name:")?,
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

    let listing_file_name = get_file_name(&listing_name);
    let listing_dir = src_path.join(&module_name).join("libraries");
    let listing_file = listing_dir.join(format!("process.{listing_file_name}.list.ts"));

    if listing_file.exists() {
        return Ok(());
    }

    fs::create_dir_all(&listing_dir).with_context(|| {
        format!(
            "Failed to create listing directory at {}",
            get_relative_path(&listing_dir)
        )
        .red()
    })?;

    let content = LISTING_STUB
        .replace("{{ListingName}}", &listing_name)
        .replace("{{EntityName}}", &entity_name);

    fs::write(&listing_file, content).with_context(|| {
        format!(
            "Failed to write lisring file at {}",
            get_relative_path(&listing_file)
        )
        .red()
    })?;

    println!(
        "{} {}",
        "✔ Listing created:".green(),
        get_relative_path(&listing_file)
    );

    Ok(())
}

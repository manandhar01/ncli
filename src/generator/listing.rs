use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt},
};

const LISTING_STUB: &str = include_str!("../../templates/process.list.stub");
const BUSINESS_LISTING_STUB: &str = include_str!("../../templates/business.process.list.stub");
const USER_LISTING_STUB: &str = include_str!("../../templates/user.process.list.stub");

pub fn run(
    listing_name: Option<&str>,
    entity_name: Option<&str>,
    module_name: Option<&str>,
    entity_type: Option<&str>,
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
        generate_listing(&listing_name, &entity_name, &module_name, t, &src_path)?;
    }

    Ok(())
}

fn generate_listing(
    listing_name: &str,
    entity_name: &str,
    module_name: &str,
    entity_type: &str,
    src_path: &std::path::Path,
) -> Result<()> {
    let listing_file_name = get_file_name(listing_name);
    let listing_dir = src_path.join(module_name).join("libraries");
    let listing_file =
        listing_dir.join(format!("process.{entity_type}{listing_file_name}.list.ts"));

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

    let template = match entity_type {
        "business." => BUSINESS_LISTING_STUB,
        "user." => USER_LISTING_STUB,
        _ => LISTING_STUB,
    };

    let content = template
        .replace("{{ListingName}}", listing_name)
        .replace("{{EntityName}}", entity_name);

    fs::write(&listing_file, content).with_context(|| {
        format!(
            "Failed to write listing file at {}",
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

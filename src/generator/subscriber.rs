use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt},
};

const SUBSCRIBER_STUB: &str = include_str!("../../templates/subscriber.stub");

pub fn run(
    subscriber_name: Option<&str>,
    entity_name: Option<&str>,
    module_name: Option<&str>,
) -> Result<()> {
    let subscriber_name = match subscriber_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter subscriber name:")?,
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

    let subscriber_file_name = get_file_name(&subscriber_name);
    let subscriber_dir = src_path.join(&module_name).join("subscribers");
    let subscriber_file = subscriber_dir.join(format!("{subscriber_file_name}.subscriber.ts"));

    if subscriber_file.exists() {
        return Ok(());
    }

    fs::create_dir_all(&subscriber_dir).with_context(|| {
        format!(
            "Failed to create subscriber directory at {}",
            get_relative_path(&subscriber_dir)
        )
        .red()
    })?;

    let content = SUBSCRIBER_STUB
        .replace("{{SubscriberName}}", &subscriber_name)
        .replace("{{EntityName}}", &entity_name);

    fs::write(&subscriber_file, content).with_context(|| {
        format!(
            "Failed to write subscriber file at {}",
            get_relative_path(&subscriber_file)
        )
        .red()
    })?;

    println!(
        "{} {}",
        "✔ Subscriber created:".green(),
        get_relative_path(&subscriber_file)
    );

    Ok(())
}

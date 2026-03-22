use anyhow::Result;
use colored::*;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
use walkdir::WalkDir;

pub fn run() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let src_path = cwd.join("src");
    let config_dir = src_path.join("config");
    let output_file = config_dir.join("column.mappings.ts");

    if !src_path.exists() {
        eprintln!("\n{}\n", "✖ src directory not found".red().bold());
        std::process::exit(1);
    }

    println!(
        "\n{}\n",
        "🔍 Scanning entity files to generate column mappings..."
            .bold()
            .cyan()
    );

    let re_entity = Regex::new(r#"@Entity\(['"]([^'"]+)['"]\)"#)?;
    let re_class = Regex::new(r#"export\s+class\s+(\w+)"#)?;
    let re_extends_common = Regex::new(r#"export\s+class\s+\w+\s+extends\s+CommonEntity"#)?;
    let re_column = Regex::new(
        r#"@(?:PrimaryGeneratedColumn|CreateDateColumn|UpdateDateColumn|DeleteDateColumn|Column)[\s\S]*?\)\s*\n\s*(?:public|protected|private)?\s*([\w_]+)\s*:\s*([A-Za-z0-9_<>]+)"#,
    )?;

    let mut common_columns: BTreeMap<String, String> = BTreeMap::new();
    let mut entity_mappings: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();

    for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.file_name().and_then(|s| s.to_str()) == Some("common.entity.ts") {
            let content = fs::read_to_string(path)?;

            for cap in re_column.captures_iter(&content) {
                let column_name = cap.get(1).unwrap().as_str();
                let ts_type = cap.get(2).unwrap().as_str();
                let mapped_type = map_ts_type(ts_type);

                common_columns.insert(column_name.to_string(), mapped_type);
            }

            println!(
                "{} Found CommonEntity with {} inherited columns",
                "✔".green().bold(),
                common_columns.len()
            );
        }
    }

    for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("ts")
            && path
                .file_name()
                .and_then(|s| s.to_str())
                .map(|n| n.ends_with(".entity.ts"))
                .unwrap_or(false)
            && path.file_name().and_then(|s| s.to_str()) != Some("common.entity.ts")
        {
            let content = fs::read_to_string(path)?;

            let table_name = re_entity
                .captures(&content)
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().to_string());

            let class_name = re_class
                .captures(&content)
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().to_string());

            if let (Some(table), Some(class)) = (table_name, class_name) {
                let input = format!("{table}:{class}");
                let mut hasher = Sha256::new();
                hasher.update(input.as_bytes());
                let hash_bytes = hasher.finalize();
                let hash = format!("{hash_bytes:x}");
                let hash = hash[..32].to_string();

                let mut columns = BTreeMap::new();
                let extends_common = re_extends_common.is_match(&content);

                if extends_common {
                    for (col, typ) in &common_columns {
                        columns.insert(col.clone(), typ.clone());
                    }
                }

                for cap in re_column.captures_iter(&content) {
                    let column_name = cap.get(1).unwrap().as_str();
                    let ts_type = cap.get(2).unwrap().as_str();
                    let mapped_type = map_ts_type(ts_type);

                    columns.insert(column_name.to_string(), mapped_type);
                }

                if !columns.is_empty() {
                    println!(
                        "{} {} => {} ({} columns)",
                        "✔".green().bold(),
                        class.bold(),
                        hash.dimmed(),
                        columns.len()
                    );

                    entity_mappings.insert(hash, columns);
                }
            }
        }
    }

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let mut ts_content = String::from("// Auto-generated file. Do not edit manually.\n\n");
    ts_content.push_str("export const columnMappings = {\n");

    for (hash, columns) in &entity_mappings {
        ts_content.push_str(&format!("    \"{}\": {{\n", hash));

        for (column, col_type) in columns {
            ts_content.push_str(&format!("        {}: '{}',\n", column, col_type));
        }

        ts_content.push_str("    },\n");
    }

    ts_content.push_str("};\n");

    fs::write(&output_file, ts_content)?;

    println!(
        "\n{} {}\n",
        "✨ Done!".green().bold(),
        format!(
            "Column mappings written to {}",
            output_file.strip_prefix(&cwd)?.display()
        )
        .cyan()
    );

    Ok(())
}

fn map_ts_type(ts_type: &str) -> String {
    match ts_type {
        "string" => "string".into(),
        "number" => "number".into(),
        "boolean" => "boolean".into(),
        "Date" => "string".into(),
        "Decimal" => "Decimal".into(),
        _ => "any".into(),
    }
}

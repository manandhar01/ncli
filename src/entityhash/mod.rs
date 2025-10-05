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
    let output_file = config_dir.join("entity.hash.ts");

    if !src_path.exists() {
        eprintln!("\n{}\n", "✖ src directory not found".red().bold());
        std::process::exit(1);
    }

    let mut hashes: BTreeMap<String, String> = BTreeMap::new();

    println!(
        "\n{}\n",
        "🔍 Scanning for entity files to generate hash..."
            .bold()
            .cyan()
    );

    let re_entity = Regex::new(r#"@Entity\(['"]([^'"]+)['"]\)"#)?;
    let re_class = Regex::new(r#"export\s+class\s+(\w+)"#)?;

    for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("ts")
            && path
                .file_name()
                .and_then(|s| s.to_str())
                .map(|n| n.ends_with(".entity.ts"))
                .unwrap_or(false)
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

                println!(
                    "{} {} ({}) => {}",
                    "✔".green().bold(),
                    class.bold(),
                    table.yellow(),
                    hash.dimmed()
                );
                hashes.insert(class.clone(), hash);
            } else {
                eprintln!(
                    "{} {:?}",
                    "⚠ Skipped".yellow(),
                    path.file_name().unwrap_or_default()
                );
            }
        }
    }

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let mut ts_content = String::from("// Auto-generated file. Do not edit manually.\n\n");
    ts_content.push_str("export const entityHash = {\n");

    for (class, hash) in &hashes {
        ts_content.push_str(&format!("    {class}: \"{hash}\",\n"));
    }

    ts_content.push_str("} as const;\n");

    fs::write(&output_file, ts_content)?;
    println!(
        "\n{} {}\n",
        "✨ Done!".green().bold(),
        format!(
            "Hashes written to {}",
            output_file.strip_prefix(&cwd)?.display()
        )
        .cyan()
    );

    Ok(())
}

use anyhow::Result;
use colored::*;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
use walkdir::WalkDir;

pub fn run() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let src_path = cwd.join("src");
    let config_dir = src_path.join("config");
    let output_file = config_dir.join("job.registry.ts");

    if !src_path.exists() {
        eprintln!("\n{}\n", "✖ src directory not found".red().bold());
        std::process::exit(1);
    }

    let mut job_map: BTreeMap<String, String> = BTreeMap::new();
    let mut import_map: Vec<(String, String)> = Vec::new();

    println!(
        "\n{}\n",
        "🔍 Scanning for job files to generate hash..."
            .bold()
            .cyan()
    );

    let re_class = Regex::new(r#"export\s+class\s+(\w+)"#)?;
    let re_super = Regex::new(r#"super\s*\("#)?;
    let re_has_super_arg = Regex::new(r#"super\s*\(['"]([^'"]+)['"]\)"#)?;

    for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("ts")
            && path
                .file_name()
                .and_then(|s| s.to_str())
                .map(|n| n.ends_with(".job.ts"))
                .unwrap_or(false)
        {
            let content = fs::read_to_string(path)?;
            if !re_super.is_match(&content) {
                println!(
                    "{} {} (no super call, skipped)",
                    "⚠".yellow(),
                    path.strip_prefix(&cwd)?.display()
                );
                continue;
            }

            let class_name = re_class
                .captures(&content)
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().to_string());

            if let Some(class_name) = class_name {
                let module_name = detect_module_name(path).unwrap_or_else(|| "unknown".to_string());
                let input = format!("{module_name}:{class_name}");
                let mut hasher = Sha256::new();
                hasher.update(input.as_bytes());
                let hash_bytes = hasher.finalize();
                let hash = format!("{hash_bytes:x}");
                let hash = hash[..32].to_string();

                let updated_content = if re_has_super_arg.is_match(&content) {
                    re_has_super_arg
                        .replace(&content, format!("super(\"{hash}\")"))
                        .to_string()
                } else {
                    re_super
                        .replace(&content, format!("super(\"{hash}\")"))
                        .to_string()
                };

                fs::write(path, updated_content)?;

                let relative_import = path
                    .strip_prefix(&src_path)
                    .unwrap()
                    .with_extension("")
                    .to_str()
                    .unwrap()
                    .replace('\\', "/");

                import_map.push((class_name.clone(), relative_import.clone()));
                job_map.insert(hash.clone(), class_name.clone());

                println!(
                    "{} {} ({}) => {}",
                    "✔".green().bold(),
                    class_name.bold(),
                    module_name.yellow(),
                    hash.dimmed()
                );
            }
        }
    }

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let mut ts_content = String::from("// Auto-generated file. Do not edit manually.\n\n");

    for (class, import_path) in &import_map {
        ts_content.push_str(&format!("import {{  {class} }} from '../{import_path}';\n"));
    }

    ts_content.push_str("\nexport const jobRegistry: Record<string, any> = {\n");

    for (hash, class) in &job_map {
        ts_content.push_str(&format!("    \"{hash}\": {class},\n"));
    }

    ts_content.push_str("};\n");

    fs::write(&output_file, ts_content)?;

    println!(
        "\n{} {}\n",
        "✨ Done!".green().bold(),
        format!(
            "Job registry written to {}",
            output_file.strip_prefix(&cwd)?.display()
        )
        .cyan()
    );

    Ok(())
}

fn detect_module_name(path: &Path) -> Option<String> {
    let path = path.canonicalize().ok()?;
    let components: Vec<String> = path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();

    if let Some(src_index) = components.iter().position(|c| c == "src") {
        return components.get(src_index + 1).cloned();
    }

    None
}

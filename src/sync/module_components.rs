use anyhow::Result;
use colored::*;
use regex::Regex;
use std::{collections::BTreeMap, fs};
use walkdir::WalkDir;

const COMPONENT_DIRS: &[&str] = &[
    "controllers",
    "dtos",
    "entities",
    "enums",
    "guards",
    "jobs",
    "libraries",
    "services",
    "strategies",
    "subscribers",
];

pub fn run() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let src_path = cwd.join("src");

    if !src_path.exists() {
        eprintln!("\n{}\n", "✖ src directory not found".red().bold());
        std::process::exit(1);
    }

    println!("\n{}\n", "🔍 Syncing module components...".bold().cyan());

    let re_class = Regex::new(r#"export\s+class\s+(\w+)"#)?;
    let re_enum = Regex::new(r#"export\s+enum\s+(\w+)"#)?;

    for entry in fs::read_dir(&src_path)? {
        let entry = entry?;
        let module_path = entry.path();

        if !module_path.is_dir() {
            continue;
        }

        let is_module = fs::read_dir(&module_path)?.filter_map(|e| e.ok()).any(|f| {
            f.path().extension().and_then(|s| s.to_str()) == Some("ts")
                && f.path()
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(|name| name.ends_with(".module.ts"))
                    .unwrap_or(false)
        });

        if !is_module {
            println!(
                "{} {} (skipped, not a module)",
                "⚠".yellow(),
                module_path.file_name().unwrap().to_string_lossy()
            );
            continue;
        }

        let module_name = module_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let mut components: BTreeMap<&str, Vec<(String, String)>> = BTreeMap::new();

        for dir in COMPONENT_DIRS {
            components.insert(dir, Vec::new());
        }

        for dir in COMPONENT_DIRS {
            let subdir = module_path.join(dir);
            if !subdir.exists() {
                continue;
            }

            for file in WalkDir::new(&subdir).into_iter().filter_map(|e| e.ok()) {
                let path = file.path();
                if path.extension().and_then(|s| s.to_str()) != Some("ts") {
                    continue;
                }

                let content = fs::read_to_string(path)?;
                let mut names = Vec::new();

                for cap in re_class.captures_iter(&content) {
                    names.push(cap[1].to_string());
                }
                for cap in re_enum.captures_iter(&content) {
                    names.push(cap[1].to_string());
                }

                if names.is_empty() {
                    continue;
                }

                let relative_path = path
                    .strip_prefix(&module_path)
                    .unwrap()
                    .with_extension("")
                    .to_str()
                    .unwrap()
                    .replace('\\', "/");

                for name in names {
                    components
                        .get_mut(dir)
                        .unwrap()
                        .push((name.clone(), relative_path.clone()));
                }
            }
        }

        for vec in components.values_mut() {
            vec.sort_by(|a, b| a.0.cmp(&b.0));
        }

        let mut ts_content = String::from("// Auto-generated file. Do not edit manually.\n\n");

        for entries in components.values() {
            for (name, path) in entries {
                ts_content.push_str(&format!("import {{ {name} }} from './{path}';\n"));
            }
        }

        ts_content.push('\n');

        let const_name = format!("{module_name}ModuleComponents");
        ts_content.push_str(&format!("export const {const_name} = {{\n"));

        for dir in COMPONENT_DIRS {
            let entries = &components[*dir];
            if entries.is_empty() {
                ts_content.push_str(&format!("    {dir}: [],\n"));
            } else {
                let class_list = entries
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                ts_content.push_str(&format!("    {dir}: [{class_list}],\n"));
            }
        }

        ts_content.push_str("};\n");

        let output_file = module_path.join(format!("{module_name}.module.components.ts"));
        fs::write(&output_file, ts_content)?;

        println!(
            "{} {} => {}",
            "✔".green().bold(),
            module_name.bold(),
            output_file.strip_prefix(&cwd)?.display()
        );
    }

    println!(
        "\n{}\n",
        "✨ All module components synced successfully!"
            .green()
            .bold()
    );

    Ok(())
}

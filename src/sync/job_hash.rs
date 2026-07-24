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
    let re_has_super_arg = Regex::new(r#"super\s*\([^)]*\)"#)?;

    for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !is_job_file(path) {
            continue;
        }

        let content = fs::read_to_string(path)?;
        if !re_super.is_match(&content) {
            warn_unregistrable_job(path, &cwd)?;
            continue;
        }

        let Some(class_name) = re_class
            .captures(&content)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
        else {
            continue;
        };

        let module_name =
            detect_module_name(path, &src_path).unwrap_or_else(|| "unknown".to_string());
        let hash = compute_job_hash(&module_name, &class_name);

        let updated_content = rewrite_super_call(&content, &hash, &re_super, &re_has_super_arg);
        fs::write(path, updated_content)?;

        let relative_import = path
            .strip_prefix(&src_path)
            .unwrap()
            .with_extension("")
            .to_str()
            .unwrap()
            .replace('\\', "/");

        import_map.push((class_name.clone(), relative_import));
        job_map.insert(hash.clone(), class_name.clone());

        println!(
            "{} {} ({}) => {}",
            "✔".green().bold(),
            class_name.bold(),
            module_name.yellow(),
            hash.dimmed()
        );
    }

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let ts_content = render_registry(&import_map, &job_map);
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

fn is_job_file(path: &Path) -> bool {
    path.extension().and_then(|s| s.to_str()) == Some("ts")
        && path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|n| n.ends_with(".job.ts"))
            .unwrap_or(false)
}

/// A `.job.ts` file with no `super()` call cannot carry a hash. Real jobs live
/// under a `jobs/` directory, so surface a loud, actionable error for those (they
/// would otherwise vanish from the registry and fail at runtime as an unknown job
/// type). Helper/abstract base classes like `common/libraries/common.job.ts` also
/// match the filename but legitimately have no `super()` call — skip them quietly.
fn warn_unregistrable_job(path: &Path, cwd: &Path) -> Result<()> {
    let in_jobs_dir = path.components().any(|c| c.as_os_str() == "jobs");
    if in_jobs_dir {
        eprintln!(
            "{} {} — no super() call found; this job is EXCLUDED from the \
             job registry and will fail at runtime as an unknown job type. \
             Add a `super('<hash>')` call to its constructor and re-run.",
            "✖".red().bold(),
            path.strip_prefix(cwd)?.display()
        );
    }
    Ok(())
}

fn compute_job_hash(module_name: &str, class_name: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{module_name}:{class_name}").as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    hash[..32].to_string()
}

fn rewrite_super_call(
    content: &str,
    hash: &str,
    re_super: &Regex,
    re_has_super_arg: &Regex,
) -> String {
    let replacement = format!("super(\"{hash}\")");
    let re = if re_has_super_arg.is_match(content) {
        re_has_super_arg
    } else {
        re_super
    };
    re.replace(content, replacement).to_string()
}

fn render_registry(import_map: &[(String, String)], job_map: &BTreeMap<String, String>) -> String {
    let mut ts_content = String::from("// Auto-generated file. Do not edit manually.\n\n");

    for (class, import_path) in import_map {
        ts_content.push_str(&format!("import {{  {class} }} from '../{import_path}';\n"));
    }

    ts_content.push_str("\nexport const jobRegistry: Record<string, any> = {\n");

    for (hash, class) in job_map {
        ts_content.push_str(&format!("    \"{hash}\": {class},\n"));
    }

    ts_content.push_str("};\n");
    ts_content
}

fn detect_module_name(path: &Path, src_path: &Path) -> Option<String> {
    path.strip_prefix(src_path)
        .ok()?
        .components()
        .next()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
}

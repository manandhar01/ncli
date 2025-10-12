use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::generator::{
    module,
    utility::{get_file_name, get_relative_path, prompt},
};

const JOB_DEFAULT_STUB: &str = include_str!("../../templates/job.stub");
const JOB_ENTITY_STUB: &str = include_str!("../../templates/entity.job.stub");

pub fn run(
    job_name: Option<&str>,
    module_name: Option<&str>,
    job_type: Option<&str>,
) -> Result<()> {
    let job_name = match job_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter job name:")?,
    };
    let module_name = match module_name {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => prompt("Enter module name:")?,
    };

    let cwd =
        std::env::current_dir().with_context(|| "Failed to get current working directory".red())?;
    let src_path = cwd.join("src");

    module::run(Some(&module_name))?;

    let job_file_name = get_file_name(&job_name);
    let job_dir = src_path.join(&module_name).join("jobs");
    let job_file = job_dir.join(format!("{job_file_name}.job.ts"));

    if job_file.exists() {
        return Ok(());
    }

    fs::create_dir_all(&job_dir).with_context(|| {
        format!(
            "Failed to create job directory at {}",
            get_relative_path(&job_dir)
        )
        .red()
    })?;

    let template = match job_type {
        Some(_) => JOB_ENTITY_STUB,
        _ => JOB_DEFAULT_STUB,
    };

    let content = template.replace("{{JobName}}", &job_name);

    fs::write(&job_file, content).with_context(|| {
        format!(
            "Failed to write job file at {}",
            get_relative_path(&job_file)
        )
        .red()
    })?;

    println!(
        "{} {}",
        "✔ Job created:".green(),
        get_relative_path(&job_file)
    );

    Ok(())
}

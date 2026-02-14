use colored::*;
use std::process::Command;

pub fn run_lint() {
    println!("{}", "→ Running npm linter (npm run lint)...".cyan());

    let status = Command::new("npm").arg("run").arg("lint").status();

    match status {
        Ok(s) if s.success() => {
            println!("\n{}\n", "✔ Linting completed successfully.".green().bold());
        }
        Ok(s) => {
            eprintln!(
                "\n{} (exit code: {})\n",
                "✖ Linting failed.".red().bold(),
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!("\n{} {}\n", "✖ Failed to run npm linter:".red().bold(), e);
        }
    }
}

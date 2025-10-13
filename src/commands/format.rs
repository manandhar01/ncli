use colored::*;
use std::process::Command;

pub fn run_formatter() {
    println!("{}", "→ Running formatter (npm run format)...".cyan());

    let status = Command::new("npm").arg("run").arg("format").status();

    match status {
        Ok(s) if s.success() => {
            println!(
                "\n{}\n",
                "✔ Formatting completed successfully.".green().bold()
            );
        }
        Ok(s) => {
            eprintln!(
                "\n{} (exit code: {})\n",
                "✖ Formatting failed.".red().bold(),
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!(
                "\n{} {}\n",
                "✖ Failed to run npm formatter:".red().bold(),
                e
            );
        }
    }
}

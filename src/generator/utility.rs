use anyhow::Result;
use colored::*;
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

pub fn prompt(msg: &str) -> Result<String> {
    print!("{} ", msg.cyan());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn to_camel_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn get_file_name(name: &str) -> String {
    let mut result = String::new();

    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('.');
            }
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

pub fn get_relative_path(path: &Path) -> String {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let src_path = cwd.join("src");

    path.strip_prefix(&src_path)
        .map(|p| format!("src/{}", p.display()))
        .unwrap_or_else(|_| path.display().to_string())
}

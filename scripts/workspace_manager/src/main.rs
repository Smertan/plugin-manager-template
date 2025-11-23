use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result};
use toml_edit::DocumentMut;

const REQUIRED_MEMBERS: &[&str] = &[
    "tests/plugin_inventory",
    "tests/plugin_mods",
    "tests/plugin_tasks",
];

fn main() -> Result<()> {
    let destination = env::args_os().nth(1).map(PathBuf::from);
    let cargo_path = match destination.as_ref() {
        Some(path) if path.is_dir() => path.join("Cargo.toml"),
        Some(path) => path.clone(),
        None => PathBuf::from("Cargo.toml"),
    };
    let manifest_label = format!("{}", cargo_path.display());

    let manifest = fs::read_to_string(&cargo_path)
        .with_context(|| format!("Unable to read {}", manifest_label))?;

    let mut document = manifest
        .parse::<DocumentMut>()
        .context("Failed to parse Cargo.toml")?;

    let members = document["workspace"]["members"]
        .as_array_mut()
        .context("workspace.members must be an array")?;

    print_header(&manifest_label);

    let mut changed = false;
    for member in REQUIRED_MEMBERS {
        if members.iter().any(|entry| entry.as_str() == Some(*member)) {
            println!("✓ {member} is already included in the workspace");
        } else {
            members.push(*member);
            changed = true;
            println!("+ Added {member} to the workspace");
        }
    }

    if changed {
        fs::write(&cargo_path, document.to_string())
            .with_context(|| format!("Unable to write {}", manifest_label))?;
        println!("Cargo.toml updated successfully.");
    } else {
        println!("No changes were required.");
    }

    Ok(())
}

fn print_header(label: &str) {
    const WIDTH: usize = 80;
    let label_len = label.len();
    let fill = WIDTH.saturating_sub(label_len);
    let left = fill / 2;
    let right = fill - left;
    println!("{}{}{}", "-".repeat(left), label, "-".repeat(right));
}

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use toml::Value;

const REQUIRED_MEMBERS: &[&str] = &[
    "tests/plugin_inventory",
    "tests/plugin_mods",
    "tests/plugin_tasks",
];

const MANIFEST_PATHS: &[&str] = &[
    "{{ crate_name }}_plugin_manager/Cargo.toml",
    "tests/plugin_tasks/Cargo.toml",
    "tests/plugin_mods/Cargo.toml",
    "tests/plugin_inventory/Cargo.toml",
];

fn main() -> Result<()> {
    match parse_command()? {
        Command::EnsureMembers { destination } => ensure_workspace_members(destination)?,
        Command::RenameManifests { project_name } => rename_manifests(&project_name)?,
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

enum Command {
    EnsureMembers { destination: Option<PathBuf> },
    RenameManifests { project_name: String },
}

fn parse_command() -> Result<Command> {
    let mut args = env::args_os().skip(1);
    match args.next() {
        None => Ok(Command::EnsureMembers { destination: None }),
        Some(first) => {
            if first.to_str() == Some("rename-manifests") {
                let project_name = args
                    .next()
                    .context("rename-manifests requires a project name argument")?;
                let project_name = project_name
                    .into_string()
                    .map_err(|_| anyhow!("project name must be valid UTF-8"))?;
                if args.next().is_some() {
                    return Err(anyhow!(
                        "rename-manifests only accepts the project name argument"
                    ));
                }
                Ok(Command::RenameManifests { project_name })
            } else {
                Ok(Command::EnsureMembers {
                    destination: Some(PathBuf::from(first)),
                })
            }
        }
    }
}

fn ensure_workspace_members(destination: Option<PathBuf>) -> Result<()> {
    let cargo_path = match destination.as_ref() {
        Some(path) if path.is_dir() => path.join("Cargo.toml"),
        Some(path) => path.clone(),
        None => PathBuf::from("Cargo.toml"),
    };
    let manifest_label = format!("{}", cargo_path.display());

    let manifest = fs::read_to_string(&cargo_path)
        .with_context(|| format!("Unable to read {}", manifest_label))?;

    let mut document: Value = toml::from_str(&manifest).context("Failed to parse Cargo.toml")?;
    let workspace = document
        .get_mut("workspace")
        .and_then(Value::as_table_mut)
        .context("workspace must be a table")?;
    let members = workspace
        .get_mut("members")
        .and_then(Value::as_array_mut)
        .context("workspace.members must be an array")?;

    print_header(&manifest_label);

    let mut changed = false;
    for member in REQUIRED_MEMBERS {
        if members.iter().any(|entry| entry.as_str() == Some(*member)) {
            println!("✓ {member} is already included in the workspace");
        } else {
            members.push(Value::from(*member));
            changed = true;
            println!("+ Added {member} to the workspace");
        }
    }

    if changed {
        let formatted =
            toml::to_string_pretty(&document).context("Failed to serialize Cargo.toml")?;
        fs::write(&cargo_path, formatted)
            .with_context(|| format!("Unable to write {}", manifest_label))?;
        println!("Cargo.toml updated successfully.");
    } else {
        println!("No changes were required.");
    }

    Ok(())
}

fn rename_manifests(project_name: &str) -> Result<()> {
    for manifest in MANIFEST_PATHS {
        update_manifest(project_name, Path::new(manifest))?;
    }
    Ok(())
}

fn update_manifest(project_name: &str, manifest_path: &Path) -> Result<()> {
    let manifest_label = format!("{}", manifest_path.display());
    let manifest = fs::read_to_string(manifest_path)
        .with_context(|| format!("Unable to read {}", manifest_label))?;
    let mut document: Value =
        toml::from_str(&manifest).with_context(|| format!("Failed to parse {}", manifest_label))?;

    let mut changed = false;
    if update_package_name(&mut document, project_name) {
        println!("Set package name to '{project_name}' in {manifest_label}");
        changed = true;
    }

    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if rename_dependency_section(&mut document, section, project_name) {
            println!(
                "Renamed '{section}.plugin-manager' to '{section}.{project_name}' in {manifest_label}"
            );
            changed = true;
        }
    }

    if changed {
        let formatted = toml::to_string_pretty(&document)
            .with_context(|| format!("Failed to serialize {}", manifest_label))?;
        fs::write(manifest_path, formatted)
            .with_context(|| format!("Unable to write {}", manifest_label))?;
    } else {
        println!("No changes required for {}", manifest_label);
    }

    Ok(())
}

fn update_package_name(document: &mut Value, project_name: &str) -> bool {
    if let Some(package) = document.get_mut("package").and_then(Value::as_table_mut) {
        if package.get("name").and_then(Value::as_str) == Some("plugin-manager") {
            if let Some(name) = package.get_mut("name") {
                *name = Value::from(project_name);
            }
            return true;
        }
    }

    false
}

fn rename_dependency_section(document: &mut Value, section: &str, project_name: &str) -> bool {
    document
        .get_mut(section)
        .and_then(Value::as_table_mut)
        .and_then(|table| table.remove("plugin-manager").map(|entry| (table, entry)))
        .map(|(table, entry)| {
            table.insert(project_name.to_string(), entry);
        })
        .is_some()
}

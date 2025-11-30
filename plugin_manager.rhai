// Get the project name from the template variables
let project_name = variable::get("project-name").to_string();
let workspace_flag = variable::get("workspace").to_string();
let workspace_manager_manifest = "scripts/workspace_manager/Cargo.toml";

let project_name = if !project_name.replace("-", " ").to_string().contains("plugin manager") {
    project_name + "Plugin Manager"
} else {
    project_name
};

let destination_directory = env::destination_directory;
print(`Destination directory: ${destination_directory}`);

// Convert to title case and set as a new variable
let project_name_title = to_title_case(project_name);
variable::set("project-name-title", project_name_title);
variable::set("project-name-kebab-case", to_kebab_case(project_name));

fn run_workspace_manager(step, command) {
    let ok = system::command(command);
    // if !ok {
    //     throw `${step} failed; inspect the workspace_manager output above for details.`;
    // }
}

fn rename_manifests(project_name, workspace_manager_manifest) {
    let command = `cargo run --quiet --manifest-path "${workspace_manager_manifest}" -- rename-manifests "${project_name}"`;
    print("Updating Cargo manifests with workspace manager.");
    run_workspace_manager("Manifest rewrite", command);
}

fn ensure_workspace_members(destination_directory, workspace_manager_manifest) {
    let command = `cargo run --quiet --manifest-path "${workspace_manager_manifest}" -- "${destination_directory}"`;
    print("Ensuring workspace members are synchronized.");
    run_workspace_manager("Workspace member sync", command);
}

/// Updates the Cargo.toml files to the new project name and syncs workspace members when needed.
fn post_generation(
    project_name,
    workspace_flag,
    destination_directory,
    workspace_manager_manifest,
) {
    rename_manifests(
        to_kebab_case(project_name),
        workspace_manager_manifest,
    );

    if workspace_flag == "true" {
        ensure_workspace_members(destination_directory, workspace_manager_manifest);
    }
}

post_generation(
    project_name,
    workspace_flag,
    destination_directory,
    workspace_manager_manifest,
);

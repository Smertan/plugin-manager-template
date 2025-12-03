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
    // TODO: run the workspace manager command with both the project name and destination args.
    let ok = system::command(command);
    print(ok);
}

let project_name_rust = variable::get("project-name-kebab-case").to_string();
let command = `cargo run --quiet --manifest-path "${workspace_manager_manifest}" -- "${project_name_rust}" "${destination_directory}" "${workspace_flag}"`;
run_workspace_manager("Project creation", command);

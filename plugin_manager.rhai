// Get the project name from the template variables
let project_name = variable::get("project-name").to_string();

let project_name = if !project_name.replace("-", " ").to_string().contains("plugin manager") {
    project_name + "Plugin Manager"
} else {
    project_name
};

// fn get_directory_name(project_name: &str, current_name: &str) -> String {
//     let project_name_snake_case = to_snake_case(project_name);
//     let mut directory_name = current_name.to_string();
//     directory_name.make_lower();
//     directory_name
// }

// Convert to title case and set as a new variable
let project_name_title = to_title_case(project_name);
variable::set("project-name-title", project_name_title);
variable::set("project-name-kebab-case", to_kebab_case(project_name));

// Rename a directory
let old_dir = "plugin_manager";
// let new_dir = "new_directory_name";

// Check if the old directory exists
if file::exists(old_dir) {
    // Rename the directory
    file::rename(old_dir, to_snake_case(project_name).to_lower());
}

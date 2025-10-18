// Get the project name from the template variables
let project_name = variable::get("project-name").to_string();

let project_name = if !project_name.replace("-", " ").to_string().contains("plugin manager") {
    project_name + "Plugin Manager"
} else {
    project_name
};
// Convert to title case and set as a new variable
let project_name_title = to_title_case(project_name);
variable::set("project-name-title", project_name_title);
 variable::set("project-name-kebab-case", to_kebab_case(project_name));
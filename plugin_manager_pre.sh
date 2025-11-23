#!/bin/bash

PROJECT_NAME=$1
WORKSPACE_FLAG=$(echo "${2:-false}" | tr '[:upper:]' '[:lower:]')
DESTINATION_DIRECTORY=$3

echo "The $PROJECT_NAME"

# print the environment variable values
# printenv

FILES=(
    "{{ crate_name }}_plugin_manager/Cargo.toml"
    "tests/plugin_tasks/Cargo.toml"
    "tests/plugin_mods/Cargo.toml"
    "tests/plugin_inventory/Cargo.toml"
)

function update_file() {
    sed -i \
        -e "s/^name = \"plugin-manager\"/name = \"${PROJECT_NAME}\"/" \
        -e "s/^plugin-manager/${PROJECT_NAME}/" \
        "$1"
    echo "Updated '$1' with '${PROJECT_NAME}'."
}

# function update_workspace_paths() {
#     local lib_file="{{ crate_name }}_plugin_manager/src/lib.rs"
#     if [ -f "$lib_file" ]; then
#         sed -i 's#\.\./tests#../../tests#g' "$lib_file"
#         sed -i '0,/path_name\.push("target")/s#path_name\.push("target");#path_name.push("..");\
#         path_name.push("target");#' "$lib_file"
#     fi

#     local unix_manifests=(
#         "tests/plugin_mods/Cargo.toml"
#         "tests/plugin_mods/Cargo-macos.toml"
#     )
#     for manifest in "${unix_manifests[@]}"; do
#         if [ -f "$manifest" ]; then
#             sed -i 's#\.\./target/release#../../target/release#g' "$manifest"
#         fi
#     done

#     local windows_manifest="tests/plugin_mods/Cargo-windows.toml"
#     if [ -f "$windows_manifest" ]; then
#         sed -i 's#\.\.\\\\target\\\\release#..\\\\..\\\\target\\\\release#g' "$windows_manifest"
#     fi
# }

function ensure_workspace_members() {
    local manifest_path="scripts/workspace_manager/Cargo.toml"
    if [ ! -f "$manifest_path" ]; then
        echo "workspace_manager manifest not found at '$manifest_path'."
        return
    fi

    if ! command -v cargo >/dev/null 2>&1; then
        echo "cargo command not found; skipping workspace member sync."
        return
    fi

    echo "Ensuring required workspace members are present in '$DESTINATION_DIRECTORY'."
    cargo run --quiet --manifest-path "$manifest_path" -- "$DESTINATION_DIRECTORY"
    if ! cargo run --quiet --manifest-path "$manifest_path" -- "$DESTINATION_DIRECTORY"; then
        echo "Failed to run workspace_manager; please check the Rust toolchain." >&2
    fi
}

echo "Checking for required files:"

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "File '$file' exists."
        update_file "$file"
    else
        echo "File '$file' does not exist."
        exit 1
    fi
done

if [ "$WORKSPACE_FLAG" == "true" ]; then
    echo "Applying workspace path overrides."
    # update_workspace_paths
    ensure_workspace_members
fi

echo "All required files exist."

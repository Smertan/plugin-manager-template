#!/bin/bash

PROJECT_NAME=$1

echo "The $PROJECT_NAME"

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

echo "All required files exist."
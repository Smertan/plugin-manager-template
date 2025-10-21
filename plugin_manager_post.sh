#!/bin/bash


PROJECT_NAME="place_holder"

FILES=(
    "${PROJECT_NAME}_plugin_manager/Cargo.toml"
    "tests/plugin_tasks/Cargo.toml"
    "tests/plugin_mods/Cargo.toml"
    "tests/plugin_inventory/Cargo.toml"
)

PROJECT_NAME="nornir-f"

function update_file() {
    sed -i \
        -e "s/^name = \"plugin-manager\"/name = \"${PROJECT_NAME}-plugin-manager\"/" \
        -e "s/^plugin-manager/${PROJECT_NAME}-plugin-manager/" \
        "$1"
    echo "Updated '$1' with '${PROJECT_NAME}-plugin-manager'."
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
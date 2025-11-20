import toml


def open_cargo_file(file_path):
    with open(file_path, "r") as file:
        return toml.load(file)


if __name__ == "__main__":
    cargo_file_path = "Cargo.toml"
    cargo_file = open_cargo_file(cargo_file_path)
    print("Cargo.toml".center(80, "-"))
    print(cargo_file)

    files = [
        "tests/plugin_inventory",
        "tests/plugin_mods",
        "tests/plugin_tasks",
        "milas/plugin",
    ]
    # for member in cargo_file["workspace"]["members"]:
    for file in files:
        if file in cargo_file["workspace"]["members"]:
            print(f"{file} is included in the workspace.")
        else:
            cargo_file["workspace"]["members"].append(file)

    with open(cargo_file_path, "w") as file:
        toml.dump(cargo_file, file)
        print("Cargo.toml updated successfully.")

# Plugin Manager Template

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Smertan/plugin-manager/ci.yml)

Reusable `cargo-generate` template for bootstrapping a plugin-oriented Rust workspace. It comes with a production-ready plugin manager crate, example plugins, and metadata-driven configuration so you can focus on behavior instead of wiring.

## Why use this template?

- Dynamic loading of plugins from shared libraries (`.so`, `.dll`, `.dylib`)
- Registration/deregistration APIs plus grouped plugin support
- Metadata-driven activation via the end-user’s `Cargo.toml`
- Sample plugin crates that illustrate best practices
- Automated name substitution handled by Rhai + shell hooks

## Prerequisites

- Latest stable Rust toolchain
- `cargo generate` (install with `cargo install cargo-generate`)
- A Git URL or local path to this repository

## Quick start with `cargo generate`

1. Install the tool once:

   ```sh
   cargo install cargo-generate
   ```

2. Generate a new project (replace the repo URL and name as needed):

   ```sh
   cargo generate \
     --git https://github.com/Smertan/plugin-manager \
     --branch main \
     --name game_plugins \
     --define github-username={TARGET-USERNAME} \
     --allow-commands
   ```

   - Use `--path .` instead of `--git …` when running from a local checkout.
   - `cargo-generate` will prompt for `github-username`; the value is used in the generated README badges.
   - `--allow-commands` permits the Rhai hook script to execute without prompting for confirmation to update the various toml files.

3. Change into the newly created workspace and verify everything compiles:

   ```sh
   cd game_plugins
   cargo test
   ```

## Template inputs and hooks

| Placeholder         | Source                                 | Purpose                                      |
|---------------------|----------------------------------------|----------------------------------------------|
| `project-name`      | Inferred from `--name` (e.g. `game_plugins`) | Becomes the crate/workspace identifier      |
| `github-username`   | Prompted at generation time            | Used in README shields and docs              |
| `workspace`         | Prompted (bool, default `false`)       | When `true`, adjusts test/env paths for nested workspaces |

During generation a Rhai hook (`plugin_manager.rhai`) runs the `scripts/workspace_manager` Rust helper via `cargo run`, which normalizes names across `Cargo.toml` files (the manager crate and the sample plugins in `tests/`). You do not need to run this helper manually during project creation.

If you generate the template from inside an existing workspace (so the manager crate ends up one directory deeper), answer `true` when prompted for `workspace`. The hook will rewrite the workspace Cargo.toml file adding the integration test paths.

## Maintaining workspace members

The legacy `main.py` helper has been replaced with a tiny Rust CLI located at `scripts/workspace_manager`. It makes sure the sample plugin crates (`tests/plugin_inventory`, `tests/plugin_mods`, `tests/plugin_tasks`) fixture stay listed under `[workspace].members` in the root `Cargo.toml`.

Run it whenever you add or remove workspace members to automatically reinsert the required fixtures:

```sh
cargo run --manifest-path scripts/workspace_manager/Cargo.toml
```

The tool only depends on the standard Rust toolchain, making it compatible with any environment.

## What gets generated?

```sh
.
├── {{ crate_name }}_plugin_manager        # Library crate containing PluginManager + traits
├── tests/
│   ├── plugin_inventory         # Example plugin crate
│   ├── plugin_mods              # Example plugin crate
│   └── plugin_tasks             # Example plugin crate
└── Cargo.toml                   # Workspace manifest already wired up
```

- The library crate exports `PluginManager`, `Plugin`, and helpers under `src/`.
- Example plugin crates illustrate how to compile `cdylib` artifacts and how metadata in an end-user project should map plugin names to shared objects.
- You can remove the sample crates or adapt them as fixtures for integration tests.

## Building your plugin manager library

Inside `{{ crate_name }}_plugin_manager` you will find a ready-to-publish crate. Key points:

- `src/lib.rs` documents all APIs and describes how plugins should expose a `create_plugins` function returning `Vec<Box<dyn Plugin>>`.
- `src/plugin_types.rs` defines the `Plugin` trait and supporting types.
- `src/plugin_structs.rs` holds the runtime data structures used by `PluginManager`.

Run the workspace tests at the root to validate changes:

```sh
cargo test
```

## Creating plugins

When authoring new plugins, implement the `Plugin` trait and export a factory:

```rust
use plugin_manager::plugin_types::Plugin;
use std::any::Any;

#[derive(Debug)]
struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> String {
        "my_plugin".to_string()
    }

    fn execute(&self, _context: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        println!("Executing MyPlugin");
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub fn create_plugins() -> Vec<Box<dyn Plugin>> {
    vec![Box::new(MyPlugin)]
}
```

### Plugin `Cargo.toml`

```toml
[package]
name = "my_plugin"
version = "0.1.0"
edition = "2021"

[dependencies]
plugin_manager = { path = "../{{ crate_name }}_plugin_manager" }

[lib]
name = "my_plugin"
crate-type = ["lib", "cdylib"]
```

Compile plugins with `cargo build --release` so the resulting `.so/.dll/.dylib` can be loaded at runtime.

## Extending the plugin trait with supertraits

The base `Plugin` trait (see `{{ crate_name }}_plugin_manager/src/plugin_types.rs`) already enforces `Send + Sync + Any` and defines `name`, `execute`, and an overridable `group`. You can layer additional capabilities on top by creating supertraits that extend `Plugin`. This keeps shared functionality centralized while letting specialized plugins add new required methods.

Example: define an analytics-oriented plugin type with an extra `flush_metrics` method and a default `group`:

```rust
pub trait AnalyticsPlugin: Plugin {
    fn flush_metrics(&self);

    fn group(&self) -> String {
        "AnalyticsPlugin".to_string()
    }
}
```

Concrete plugins implement the supertrait instead of the base trait directly:

```rust
pub struct MetricsPlugin;

impl Plugin for MetricsPlugin {
    fn name(&self) -> String { "metrics".into() }
    fn execute(&self, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
}

impl AnalyticsPlugin for MetricsPlugin {
    fn flush_metrics(&self) {
        println!("flushing stats");
    }
}
```

If you want the runtime to treat analytics plugins differently (for example, to expose `flush_metrics` through the `Plugins` enum), add a new enum variant and update `PluginManager`’s registration/execution paths to match:

```rust
pub enum Plugins {
    Base(Box<dyn Plugin>),
    Inventory(Box<dyn PluginInventory>),
    Analytics(Box<dyn AnalyticsPlugin>),
}
```

Because each supertrait still inherits from `Plugin`, the manager can fall back to the common `execute` flow while also opting into specialized behaviors when the variant matches.

## Wiring plugins into applications

End-user applications load plugins through `package.metadata.plugins`:

```toml
[package.metadata.plugins]
task_scheduler = "/absolute/path/to/libtask_scheduler.so"

[package.metadata.plugins.analytics]
metrics = "/path/to/libmetrics.so"
logger = "/path/to/liblogger.so"
```

At runtime:

```rust
use plugin_manager::PluginManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = PluginManager::new();
    manager = manager.activate_plugins()?;
    manager.execute_plugin("task_scheduler", &())?;
    Ok(())
}
```

## Next steps

1. Update the generated README badges with your GitHub org/repo (the placeholder is already filled if you supplied `github-username`).
2. Replace or expand the sample plugin crates with real integrations.
3. Publish the `{{ crate_name }}_plugin_manager` crate to crates.io or use it via a path/git dependency inside your applications.

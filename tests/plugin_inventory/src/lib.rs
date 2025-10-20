pub mod inventory_a;
// use plugin_manager::plugin_structs::Plugins;
use plugin_manager::plugin_types::Plugins;

#[unsafe(no_mangle)]
pub fn create_plugins() -> Vec<Plugins> {
    // let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(inventory_a::InventoryA)];
    let plugins = vec![Plugins::Inventory(Box::new(inventory_a::InventoryA))];
    plugins
}

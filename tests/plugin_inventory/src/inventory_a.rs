use plugin_manager::plugin_types::{Plugin, PluginInventory};
use std::any::Any;

#[derive(Clone, PartialEq, Eq)]
pub struct InventoryA;

impl Plugin for InventoryA {
    fn name(&self) -> String {
        String::from("inventory_a")
    }

    fn execute(&self, _context: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        println!("Executing Inventory A");
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl PluginInventory for InventoryA {
    fn load(&self) {
        println!("Executing other method in Inventory A");
    }
}

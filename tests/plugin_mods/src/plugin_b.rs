use plugin_manager::plugin_types::Plugin;
use std::any::Any;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginB;

impl Plugin for PluginB {
    fn name(&self) -> String {
        String::from("plugin_b")
    }

    fn execute(&self, _context: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        println!("Executing Plugin B");
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl PluginB {
    pub fn other_method(&self) {
        println!("Executing other method in Plugin B");
    }
}

// #[unsafe(no_mangle)]
// pub fn create_plugin() -> Box<dyn Plugin> {
//     Box::new(PluginB)
// }

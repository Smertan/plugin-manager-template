pub mod plugin_a;
pub mod plugin_b;
use plugin_manager::plugin_types::Plugins;
// use plugin_manager::plugin_structs::Plugins;

#[unsafe(no_mangle)]
pub fn create_plugins() -> Vec<Plugins> {
    // let plugins: Vec<Box<dyn Plugin>> =
    //     vec![Box::new(plugin_a::PluginA), Box::new(plugin_b::PluginB)];
    let plugins = vec![
        Plugins::Base(Box::new(plugin_a::PluginA)),
        Plugins::Base(Box::new(plugin_b::PluginB)),
    ];
    plugins
}

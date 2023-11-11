/// Main entry for the PostgreSQL plugin
use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;

#[derive(Default)]
pub struct GeyserPluginSample {}

impl GeyserPlugin for GeyserPluginSample {
    fn name(&self) -> &'static str {
        "GeyserPluginSample"
    }
    fn on_unload(&mut self) {}    
}


impl std::fmt::Debug for GeyserPluginSample {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl GeyserPluginSample {
    fn new() -> Self {
        Self {}
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = GeyserPluginSample::new();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}

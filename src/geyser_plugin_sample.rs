/// Main entry for the PostgreSQL plugin
use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;

#[derive(Default, Debug)]
pub struct GeyserPluginSample {}

impl GeyserPlugin for GeyserPluginSample {
    fn name(&self) -> &'static str {
        "GeyserPluginSample"
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

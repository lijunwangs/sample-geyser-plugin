use log::*;
/// Main entry for the Sample plugin
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions, ReplicaEntryInfoVersions,
    ReplicaTransactionInfoVersions, Result, SlotStatus,
};

use solana_sdk::clock::Slot;

#[derive(Default)]
pub struct GeyserPluginSample {}

impl GeyserPlugin for GeyserPluginSample {
    fn name(&self) -> &'static str {
        "GeyserPluginSample"
    }
    fn on_load(&mut self, config_file: &str) -> Result<()> {
        // the following code causes unload issue -- the plugin library is not unloaded from the memory
        solana_logger::setup_with_default("info");
        info!(
            "Loading plugin {:?} from config_file {:?}",
            self.name(),
            config_file
        );

        Ok(())
    }

    fn on_unload(&mut self) {
    }

    #[allow(unused_variables)]
    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: Slot,
        is_startup: bool,
    ) -> Result<()> {
        info!("Got account notification");
        Ok(())
    }

    fn update_slot_status(&self, slot: u64, parent: Option<u64>, status: SlotStatus) -> Result<()> {
        info!("Updating slot {slot:?} at with status {status:?} of parent {parent:?}");
        Ok(())
    }

    fn notify_block_metadata(&self, _blockinfo: ReplicaBlockInfoVersions) -> Result<()> {
        info!("Got block metadata");
        Ok(())
    }

    /// Called when an entry is executed.
    #[allow(unused_variables)]
    fn notify_entry(&self, entry: ReplicaEntryInfoVersions) -> Result<()> {
        info!("Got entry notification");
        Ok(())
    }

    #[allow(unused_variables)]
    fn notify_transaction(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        slot: Slot,
    ) -> Result<()> {
        info!("Got txn notification");
        Ok(())
    }

    /// Check if the plugin is interested in account data
    /// Default is true -- if the plugin is not interested in
    /// account data, please return false.
    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    /// Check if the plugin is interested in transaction data
    /// Default is false -- if the plugin is interested in
    /// transaction data, please return true.
    fn transaction_notifications_enabled(&self) -> bool {
        true
    }

    /// Check if the plugin is interested in entry data
    /// Default is false -- if the plugin is interested in
    /// entry data, return true.
    fn entry_notifications_enabled(&self) -> bool {
        true
    }
}

impl std::fmt::Debug for GeyserPluginSample {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl GeyserPluginSample {
    fn new() -> Self {
        Self::default()
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

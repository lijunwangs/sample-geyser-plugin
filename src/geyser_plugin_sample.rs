/// Main entry for the Sample plugin
use {
    crossbeam_channel::{bounded, Receiver, RecvTimeoutError, Sender},
    log::info,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
        ReplicaEntryInfoVersions, ReplicaTransactionInfoVersions, Result, SlotStatus,
    },
    solana_sdk::clock::Slot,
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread::{Builder, JoinHandle},
        time::Duration,
    },
};

#[derive(Default)]
pub struct GeyserPluginSample {
    worker: Option<JoinHandle<()>>,
    exit: Arc<AtomicBool>,
    sender: Option<Sender<i32>>,
}

fn do_work(exit_worker: Arc<AtomicBool>, receiver: Receiver<i32>) {
    while !exit_worker.load(Ordering::Relaxed) {
        let work = receiver.recv_timeout(Duration::from_millis(500));
        match work {
            Ok(work) => {
                info!("Got work {work}");
            }
            Err(err) => match err {
                RecvTimeoutError::Timeout => {
                    info!("Timed out");
                    continue;
                }
                _ => {
                    info!("Got error {err:?}");
                    break;
                }
            }
        }
    }
}

impl GeyserPlugin for GeyserPluginSample {
    fn name(&self) -> &'static str {
        "GeyserPluginSample"
    }

    #[allow(unused_variables)]
    fn setup_logger(&self, logger: &'static dyn log::Log, level: log::LevelFilter) -> Result<()> {
        log::set_max_level(level);
        if let Err(err) = log::set_logger(logger) {
            return Err(GeyserPluginError::Custom(Box::new(err)));
        }
        Ok(())
    }

    fn on_load(&mut self, config_file: &str) -> Result<()> {
        // the following code causes unload issue -- the plugin library is not unloaded from the memory
        // https://github.com/rust-lang/log/issues/421
        // env_logger::init();
        info!(
            "Loading plugin {:?} from config_file {:?}",
            self.name(),
            config_file
        );
        Ok(())
    }

    fn on_unload(&mut self) {
        // The following crashes at the exit.
        // env_logger::init_from_env(env_logger::Env::default().default_filter_or("off"));
        self.exit.store(true, Ordering::Relaxed);
        if let Some(worker) = self.worker.take() {
            worker.join().unwrap();
        }
    }

    #[allow(unused_variables)]
    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: Slot,
        is_startup: bool,
    ) -> Result<()> {
        info!("Got account notification");
        if let Some(sender) = &self.sender {
            sender.send(slot as i32).unwrap();
        }
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
        let (sender, receiver) = bounded(40960);
        let exit = Arc::new(AtomicBool::default());
        let exit_clone = exit.clone();
        let worker = Builder::new()
            .name(format!("sample"))
            .spawn(move || -> () {
                do_work(exit_clone, receiver);
            })
            .unwrap();

        Self {
            worker: Some(worker),
            exit,
            sender: Some(sender),
        }
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

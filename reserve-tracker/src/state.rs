use snarkvm::prelude::VM;
use snarkvm::ledger::store::helpers::memory::ConsensusMemory;
use tokio::sync::mpsc;

use crate::CurrentNetwork;

pub struct AppState {
    pub addresses: Vec<String>,
    pub task_tx: mpsc::Sender<super::background::BackgroundTaskMsg>,
}

// Declare a thread-local static
thread_local! {
    pub static VM_INSTANCE: std::cell::RefCell<Option<VM<CurrentNetwork, ConsensusMemory<CurrentNetwork>>>> = std::cell::RefCell::new(None);
}

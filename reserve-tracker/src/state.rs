use snarkvm::prelude::VM;
use snarkvm::ledger::store::helpers::memory::ConsensusMemory;
use tokio::sync::mpsc;

use crate::CurrentNetwork;
use crate::background::BackgroundTaskMsg;

use once_cell::sync::OnceCell;
use std::sync::Mutex;

pub static VM_GLOBAL: OnceCell<Mutex<VM<CurrentNetwork, ConsensusMemory<CurrentNetwork>>>> = OnceCell::new();

pub struct AppState {
    pub addresses: Vec<String>,
    pub transactions: Vec<String>,
    pub task_tx: mpsc::Sender<BackgroundTaskMsg>,
    pub private_key: String,
    pub endpoint: String,
    pub transactions_file: String,
}



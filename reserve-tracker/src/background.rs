use std::borrow::BorrowMut;
use std::str::FromStr;
use snarkvm::prelude::store::ConsensusStore;
use snarkvm::prelude::store::helpers::memory::ConsensusMemory;
use snarkvm::prelude::{Network, Program, VM};
use tokio::sync::mpsc;
use tokio::time::{interval, Interval, Duration};
use tokio::task::spawn_blocking;
use crate::CurrentNetwork;

use crate::state::VM_INSTANCE;

pub enum BackgroundTaskMsg {
    RunNow,
    Shutdown,
}

pub fn spawn_background_task(mut rx: mpsc::Receiver<BackgroundTaskMsg>, cadence: u64) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(cadence));
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    if let Err(e) = run_cpu_intensive_task().await {
                        eprintln!("Failed to run background task: {}", e);
                    }
                }
                msg = rx.recv() => {
                    match msg {
                        Some(BackgroundTaskMsg::RunNow) => {
                            if let Err(e) = run_cpu_intensive_task().await {
                                eprintln!("Failed to run background task: {}", e);
                            }
                        }
                        Some(BackgroundTaskMsg::Shutdown) | None => {
                            // Exit the loop on shutdown or channel closed
                            break;
                        }
                    }
                }
            }
        }
        println!("Background task shutting down gracefully.");
    })
}

async fn prove_for_address() -> Result<<CurrentNetwork as Network>::TransactionID, String> {
    let transaction_id = VM_INSTANCE.with(|obj_cell| {
        let mut maybe_obj = obj_cell.borrow_mut();

        // Initialize the VM with the `proof_of_reserves` program.
        if maybe_obj.is_none() {
            // Initialize an RNG.
            let rng = rand::rngs::OsRng();
            // Initialize the VM.
            let vm = VM::from(ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None).unwrap()).unwrap();
            // Load the program.
            let program = Program::from_str(include_str!("../../proof_of_reserves/build/main.aleo")).unwrap();
            let deployment = vm.process().read().deploy(&program, rng).unwrap();
            vm.process().write().load_deployment(&deployment).unwrap();
        }

        // Run the VM.
        let vm = maybe_obj.as_mut().unwrap();

    };

    Ok(transaction)
}





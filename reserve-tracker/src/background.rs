use std::str::FromStr;
use snarkvm::prelude::store::ConsensusStore;
use snarkvm::prelude::store::helpers::memory::ConsensusMemory;
use snarkvm::prelude::{Address, Plaintext, Literal, PrivateKey, Program, Value, VM, Transaction};
use snarkvm::prelude::query::Query;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tokio::task::spawn_blocking;

use crate::{CurrentAleo, CurrentNetwork, state::AppState, state::VM_GLOBAL};
use std::sync::{Arc, Mutex};
use tokio::sync::{RwLock};
use crate::utilities::broadcast_transaction;

pub enum BackgroundTaskMsg {
    RunNow,
    Shutdown,
}

pub fn spawn_background_task(
    mut rx: mpsc::Receiver<BackgroundTaskMsg>,
    cadence: u64,
    app_state: Arc<RwLock<AppState>>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(cadence));
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    if let Err(e) = prove_public_balance(&app_state).await {
                        eprintln!("Failed to run background task: {}", e);
                    }
                }
                msg = rx.recv() => {
                    match msg {
                        Some(BackgroundTaskMsg::RunNow) => {
                            if let Err(e) = prove_public_balance(&app_state).await {
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

async fn prove_public_balance(app_state: &Arc<RwLock<AppState>>) -> Result<(), String> {
    let st = app_state.read().await;
    let private_key = PrivateKey::<CurrentNetwork>::from_str(&st.private_key)
        .map_err(|e| format!("Failed to parse private key: {}", e))?;

    let addresses = st.addresses.iter()
        .map(|addr_str| Address::<CurrentNetwork>::from_str(addr_str)
            .map_err(|e| format!("Failed to parse address '{}': {}", addr_str, e))
        )
        .collect::<Result<Vec<_>, _>>()?;

    let endpoint = st.endpoint.clone();
    let transactions_file = st.transactions_file.clone(); // Add this field to AppState

    drop(st);

    // Run CPU-intensive in blocking
    let transaction = spawn_blocking(move || {
        let tx = prove_for_address(addresses, private_key, &endpoint)?;
        // If broadcasting should happen inside blocking:
        // vm.broadcast(&tx).map_err(|e| format!("Failed to broadcast: {}", e))?;
        Ok::<_, String>(tx)
    })
        .await
        .map_err(|e| format!("task join error: {:?}", e))??;

    println!("Background task completed. Transaction ID: {}", transaction.id());

    // If broadcasting is I/O-bound, do it here (non-blocking)
    // broadcast_transaction(&endpoint, &transaction).await.map_err(|e| format!("Failed to broadcast: {}", e))?;

    // Store the transaction ID in state and save
    {
        let mut st = app_state.write().await;
        st.transactions.push(transaction.id().to_string());
        // Save transactions
        crate::storage::save_transactions(&st.transactions, &transactions_file).await
            .map_err(|e| format!("Failed to save transactions: {}", e))?;
    }

    Ok(())
}


fn prove_for_address(
    addresses: Vec<Address<CurrentNetwork>>,
    private_key: PrivateKey<CurrentNetwork>,
    endpoint: &str,
) -> Result<Transaction<CurrentNetwork>, String> {

    let vm = get_or_init_vm()?;
    let rng = &mut rand::rngs::OsRng;

    let first = Value::Plaintext(Plaintext::from(Literal::Address(Address::try_from(&private_key).unwrap())));
    let second = Value::Plaintext(Plaintext::Array(
        addresses.into_iter().map(|addr| Plaintext::from(Literal::Address(addr))).collect(),
        Default::default()
    ));
    let inputs = vec![first, second];
    let query = Some(Query::REST(endpoint.to_string()));

    let transaction = vm.execute(
        &private_key,
        ("proof_of_reserves_v0_1_0.aleo", "record_balances2"),
        inputs.iter(),
        None,
        0u64,
        query,
        rng
    ).map_err(|e| format!("Failed to execute VM: {}", e))?;

    // Broadcast the transaction to the endpoint.
    broadcast_transaction(&transaction, endpoint, "testnet")
        .map_err(|e| format!("Failed to broadcast transaction: {}", e))?;
    println!("Broadcasted transaction: {:?}", transaction.id());

    Ok(transaction)
}


fn get_or_init_vm() -> Result<std::sync::MutexGuard<'static, VM<CurrentNetwork, ConsensusMemory<CurrentNetwork>>>, String> {
    // Initialize VM_GLOBAL if not already done
    VM_GLOBAL.get_or_try_init(|| {
        let vm = VM::from(ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None)
            .map_err(|e| format!("Failed to open consensus store: {}", e))?)
            .map_err(|e| format!("Failed to create VM: {}", e))?;

        let program_str = include_str!("../../proof_of_reserves/build/main.aleo");
        let program = Program::from_str(program_str)
            .map_err(|e| format!("Failed to parse program: {}", e))?;

        {
            let deployment = vm.process().read()
                .deploy::<CurrentAleo, _>(&program, &mut rand::rngs::OsRng)
                .map_err(|e| format!("Failed to deploy program: {}", e))?;
            vm.process().write()
                .load_deployment(&deployment)
                .map_err(|e| format!("Failed to load deployment: {}", e))?;
        }

        Ok::<_, String>(Mutex::new(vm))
    }).map_err(|e| e.to_string())?;

    // Now VM_GLOBAL is initialized, get a guard
    Ok(VM_GLOBAL.get().unwrap().lock().unwrap())
}


use axum::{Router, routing::{get, post}};
use clap::Parser;
use std::sync::Arc;
use snarkvm::prelude::TestnetV0;
use tokio::sync::{RwLock, mpsc};
use crate::{
    storage::{load_addresses, save_addresses},
    state::AppState,
    background::{BackgroundTaskMsg, spawn_background_task},
};
use crate::routes::{get_form, get_transactions_page, handle_form, handle_transactions_form};
use crate::storage::load_transactions;

mod state;
mod storage;
mod background;
mod routes;
mod utilities;

pub type CurrentNetwork = TestnetV0;
pub type CurrentAleo = snarkvm::circuit::AleoTestnetV0;

pub const NETWORK_NAME: &str = "testnet";

/// Command-line options
#[derive(Parser, Debug)]
#[command(name = "reserve-tracker", version, about = "Address tracking server")]
struct Cli {
    /// Port to run the server on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// File to store the addresses
    #[arg(short, long, default_value = "addresses.json")]
    file: String,

    /// File to store the transactions
    #[arg(short, long, default_value = "transactions.json")]
    transactions_file: String,

    /// Cadence (in seconds) for background task
    #[arg(long, default_value_t = 12000)]
    cadence: u64,

    /// Private key (required)
    #[arg(long)]
    private_key: String,

    /// Endpoint (optional, with default)
    #[arg(long, default_value = "https://api.explorer.provable.com/v1")]
    endpoint: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Load addresses from the specified file
    let addresses = load_addresses(&cli.file).await.unwrap_or_else(|_| Vec::new());
    // Load transactions from the specified file
    let transactions = load_transactions(&cli.transactions_file).await.unwrap_or_default();

    // Channel for signaling the background task
    let (tx, rx) = mpsc::channel(10);

    let app_state = AppState {
        addresses,
        transactions,
        task_tx: tx.clone(),
        private_key: cli.private_key,
        endpoint: cli.endpoint,
        transactions_file: cli.transactions_file,
    };

    let app_state = Arc::new(RwLock::new(app_state));

    // Spawn background task with access to app_state
    let bg_handle = spawn_background_task(rx, cli.cadence, app_state.clone());

    // Setup Axum routes
    let app = Router::new()
        // Root
        .route("/", get(get_form))
        // form
        .route("/form", post(handle_form))
        .with_state(app_state.clone())
        // transactions
        .route("/transactions", get(get_transactions_page).post(handle_transactions_form))
        .with_state(app_state.clone());

    let addr = format!("0.0.0.0:{}", cli.port);
    println!("Server running on http://{}", addr);
    println!("Using storage file: {}", cli.file);
    println!("Background task cadence: {}s", cli.cadence);
    println!("Using endpoint: {}", app_state.read().await.endpoint);
    println!("Private key provided: {}", &app_state.read().await.private_key);

    // Create a shutdown signal future
    let shutdown_signal = async {
        tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };

    // Run the server with graceful shutdown
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal)
        .await
        .unwrap();

    // Server stopped, send shutdown message to background task
    let _ = tx.send(BackgroundTaskMsg::Shutdown).await;

    // Wait for background task to finish
    if let Err(e) = bg_handle.await {
        eprintln!("Background task ended with error: {:?}", e);
    }

    // Save the addresses
    if let Err(e) = save_addresses(&app_state.read().await.addresses, &cli.file).await {
        eprintln!("Failed to save addresses: {}", e);
    } else {
        println!("Addresses saved. Shutting down.");
    };
}

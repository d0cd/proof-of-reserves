use axum::{Router, routing::{get, post}};
use clap::Parser;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use snarkvm::prelude::TestnetV0;

use crate::{
    routes::{get_form, handle_form},
    storage::{load_addresses, save_addresses},
    state::AppState,
    background::{BackgroundTaskMsg, spawn_background_task},
};

mod routes;
mod state;
mod storage;
mod background;

type CurrentNetwork = TestnetV0;

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

    /// Cadence (in seconds) for background task
    #[arg(long, default_value_t = 60)]
    cadence: u64,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Load addresses from the specified file
    let addresses = load_addresses(&cli.file).await.unwrap_or_else(|_| Vec::new());

    // Channel for signaling the background task (run now, shutdown)
    let (tx, rx) = mpsc::channel(10);

    let app_state = AppState {
        addresses: addresses,
        task_tx: tx.clone(),
    };
    let app_state = Arc::new(RwLock::new(app_state));

    // Spawn background task
    let bg_handle = spawn_background_task(rx, cli.cadence);

    // Setup Axum routes
    let app = Router::new()
        .route("/", get(get_form))
        .route("/form", post(handle_form))
        .with_state(app_state.clone());

    let addr = format!("0.0.0.0:{}", cli.port);
    println!("Server running on http://{}", addr);
    println!("Using storage file: {}", cli.file);
    println!("Background task cadence: {}s", cli.cadence);

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

    // Server is stopped, now send a shutdown message to the background task
    let _ = tx.send(BackgroundTaskMsg::Shutdown).await;

    // Wait for background task to finish
    if let Err(e) = bg_handle.await {
        eprintln!("Background task ended with error: {:?}", e);
    }

    // Once the server and background task have exited, save the addresses
    if let Err(e) = save_addresses(&app_state.read().await.addresses, &cli.file).await {
        eprintln!("Failed to save addresses: {}", e);
    } else {
        println!("Addresses saved. Shutting down.");
    };
}

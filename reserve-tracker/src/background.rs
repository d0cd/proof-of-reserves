use tokio::sync::mpsc;
use tokio::time::{interval, Interval, Duration};
use tokio::task::spawn_blocking;

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

async fn run_cpu_intensive_task() -> Result<(), String> {
    // Simulate CPU-intensive work using spawn_blocking
    spawn_blocking(|| {
        // Placeholder for heavy computation
        // For example, do some heavy loop or compute hashes
        // Here we just sleep to simulate work
        std::thread::sleep(std::time::Duration::from_secs(2));
    }).await.map_err(|e| format!("task join error: {:?}", e))?;

    println!("Background task completed work.");
    Ok(())
}

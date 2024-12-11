use tokio::sync::mpsc;

pub struct AppState {
    pub addresses: Vec<String>,
    pub task_tx: mpsc::Sender<super::background::BackgroundTaskMsg>,
}

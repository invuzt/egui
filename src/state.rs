use std::sync::Arc;
use tokio::sync::Mutex;

pub struct LogEntry {
    pub ip: String,
    pub time: String,
}

pub struct AppState {
    pub server_status: String,
    pub api_hits: u64,
    pub logs: Vec<LogEntry>,
    pub show_panel: bool,
}

pub type SharedState = Arc<Mutex<AppState>>;

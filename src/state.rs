use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub server_status: String,
    pub api_hits: u64,
}

pub type SharedState = Arc<Mutex<AppState>>;

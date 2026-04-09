use std::sync::Arc;
use tokio::sync::Mutex;
use egui::Pos2;

pub struct ActiveNode {
    pub id: u64,
    pub mod_index: usize, // Telunjuk ke ModRegistry
    pub pos: Pos2,
    pub current_value: f32,
}

pub struct AppState {
    pub active_nodes: Vec<ActiveNode>,
    pub animation_time: f32,
    pub zoom: f32,
    pub pan: egui::Vec2,
}

pub type SharedState = Arc<Mutex<AppState>>;

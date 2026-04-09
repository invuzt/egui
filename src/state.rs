use egui::{Pos2, Vec2};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type SharedState = Arc<Mutex<AppState>>;

pub struct ActiveNode {
    pub id: u64,
    pub mod_index: usize,
    pub pos: Pos2,
    pub current_value: f32,
    pub parent_id: Option<u64>, // Untuk sistem drag & drop tumpuk
}

pub struct AppState {
    pub active_nodes: Vec<ActiveNode>,
    pub animation_time: f32,
    pub zoom: f32,
    pub pan: Vec2,
}

use std::sync::Arc;
use tokio::sync::Mutex;
use egui::{Pos2, Vec2};

pub struct ActiveNode {
    pub id: u64,
    pub mod_index: usize,
    pub pos: Pos2,
    pub current_value: f32,
}

pub struct Connection {
    pub from_node: u64,
    pub to_node: u64,
}

pub struct AppState {
    pub active_nodes: Vec<ActiveNode>,
    pub connections: Vec<Connection>,
    pub animation_time: f32,
    pub zoom: f32,
    pub pan: Vec2,
}

pub type SharedState = Arc<Mutex<AppState>>;

use std::sync::Arc;
use tokio::sync::Mutex;
use egui::Pos2;

#[derive(Clone, PartialEq)]
pub enum NodeType {
    InputTime,
    EasingCurve(String), // String = nama kurva (e.g., "SineInOut")
    OutputProperty(String), // e.g., "Rotation", "Opacity"
}

pub struct Node {
    pub id: u64,
    pub name: String,
    pub node_type: NodeType,
    pub pos: Pos2,
    pub value: f32, // Nilai output saat ini
}

pub struct Connection {
    pub from_node: u64,
    pub to_node: u64,
}

pub struct AppState {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub show_panel: bool,
    pub dark_mode: bool,
    pub animation_time: f32, // Waktu global animasi (0.0 - 1.0)
}

pub type SharedState = Arc<Mutex<AppState>>;

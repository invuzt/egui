use std::sync::Arc;
use tokio::sync::Mutex;
use egui::Pos2;

#[derive(Clone, Copy, PartialEq)]
pub enum EasingType {
    Linear,
    SineInOut,
    BounceOut,
    ElasticOut,
}

#[derive(Clone, PartialEq)]
pub enum NodeType {
    Generator,      // Source: Time, Frame
    Tweener(EasingType), // Logic: Easing Curves
    Actor(String),  // Output: Position, Rotation, Scale
}

pub struct Node {
    pub id: u64,
    pub name: String,
    pub node_type: NodeType,
    pub pos: Pos2,
    pub input_val: f32,
    pub output_val: f32,
}

pub struct Connection {
    pub from_node: u64,
    pub to_node: u64,
}

pub struct AppState {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub show_panel: bool,
    pub animation_time: f32, // Global Clock (0.0 to 1.0)
}

pub type SharedState = Arc<Mutex<AppState>>;

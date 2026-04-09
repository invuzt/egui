#![cfg(target_os = "android")]
mod state;
mod ui;

use state::{AppState, SharedState, Node, NodeType, EasingType, Connection};
use std::sync::Arc;
use tokio::sync::Mutex;
use egui::Pos2;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut nodes = Vec::new();
    let mut connections = Vec::new();

    // Skenario: Pulse Animation
    nodes.push(Node { id: 1, name: "🕒 Clock".into(), node_type: NodeType::Generator, pos: Pos2::new(50.0, 100.0), input_val: 0.0, output_val: 0.0 });
    nodes.push(Node { id: 2, name: "📈 Elastic".into(), node_type: NodeType::Tweener(EasingType::ElasticOut), pos: Pos2::new(200.0, 100.0), input_val: 0.0, output_val: 0.0 });
    nodes.push(Node { id: 3, name: "🎭 Scale".into(), node_type: NodeType::Actor("Scale".into()), pos: Pos2::new(350.0, 100.0), input_val: 0.0, output_val: 0.0 });

    connections.push(Connection { from_node: 1, to_node: 2 });
    connections.push(Connection { from_node: 2, to_node: 3 });

    let state: SharedState = Arc::new(Mutex::new(AppState {
        nodes, connections,
        show_panel: true,
        animation_time: 0.0,
    }));

    let mut options = eframe::NativeOptions::default();
    let app_state = state.clone();
    options.event_loop_builder = Some(Box::new(move |builder| { 
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app); 
    }));

    let _ = eframe::run_native("Odfiz Motion Canvas", options, Box::new(|cc| {
        Box::new(ui::OdfizApp::new(app_state))
    }));
}

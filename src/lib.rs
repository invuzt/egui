#![cfg(target_os = "android")]
mod state;
mod ui;

use state::{AppState, SharedState, Node, NodeType, Connection};
use std::sync::Arc;
use tokio::sync::Mutex;
use egui::Pos2;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    // --- SETUP AWAL MODULAR GRAPH ---
    let mut nodes = Vec::new();
    let mut connections = Vec::new();

    // Node 1: Input Waktu Global (0.0 - 1.0)
    nodes.push(Node { id: 1, name: "🕒 Time".into(), node_type: NodeType::InputTime, pos: Pos2::new(20.0, 20.0), value: 0.0 });
    // Node 2: Easing Curve (SineInOut)
    nodes.push(Node { id: 2, name: "📈 SineInOut".into(), node_type: NodeType::EasingCurve("SineInOut".into()), pos: Pos2::new(150.0, 20.0), value: 0.0 });
    // Node 3: Output (Akan menggerakkan rotasi di Preview)
    nodes.push(Node { id: 3, name: "⚙ Rotate".into(), node_type: NodeType::OutputProperty("Rotation".into()), pos: Pos2::new(280.0, 20.0), value: 0.0 });

    // Koneksi: Time -> Sine -> Rotate
    connections.push(Connection { from_node: 1, to_node: 2 });
    connections.push(Connection { from_node: 2, to_node: 3 });

    let state: SharedState = Arc::new(Mutex::new(AppState {
        nodes,
        connections,
        show_panel: true,
        dark_mode: true,
        animation_time: 0.0,
    }));

    // (Logika event loop eframe sama seperti sebelumnya, tanpa server tokio)
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    options.event_loop_builder = Some(Box::new(move |builder| { builder.with_android_app(app); }));
    let app_state = state.clone();
    let _ = eframe::run_native("Odfiz Motion", options, Box::new(|cc| {
        cc.egui_ctx.set_pixels_per_point(1.2);
        Box::new(ui::OdfizApp::new(app_state))
    }));
}

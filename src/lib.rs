#![cfg(target_os = "android")]
mod state;
mod ui;
mod mods;

use state::{AppState, SharedState};
use std::sync::Arc;
use tokio::sync::Mutex;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let state: SharedState = Arc::new(Mutex::new(AppState {
        active_nodes: Vec::new(),
        animation_time: 0.0,
        zoom: 1.0,
        pan: egui::Vec2::ZERO,
    }));

    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native("Odfiz Mod Engine", options, Box::new(|_cc| {
        Box::new(ui::OdfizApp::new(state))
    }));
}

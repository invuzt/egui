#![cfg(target_os = "android")]

mod state;
mod server;
mod ui;

use state::{AppState, SharedState};
use std::sync::Arc;
use tokio::sync::Mutex;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let state: SharedState = Arc::new(Mutex::new(AppState {
        server_status: "Starting...".into(),
        api_hits: 0,
        logs: Vec::new(),
        show_panel: true,
        dark_mode: true, // Inisialisasi default ke Dark Mode
    }));

    let s_state = state.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(server::run_server(s_state));
    });

    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let app_state = state.clone();
    let _ = eframe::run_native(
        "Odfiz Core",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.4);
            Box::new(ui::OdfizApp::new(app_state))
        }),
    );
}

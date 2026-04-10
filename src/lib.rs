#![cfg(target_os = "android")]
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;

struct AppState {
    is_active: bool,
    counter: u64,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let state = Arc::new(Mutex::new(AppState {
        is_active: false,
        counter: 0,
    }));

    let server_state = state.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let app = axum::Router::new()
                .route("/hit", axum::routing::get({
                    let s = server_state.clone();
                    move || async move {
                        let mut data = s.lock().await;
                        if data.is_active {
                            data.counter += 1;
                            "Data OK"
                        } else {
                            "OFFLINE"
                        }
                    }
                }));
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    });

    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Minimalist",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);
            Box::new(OdfizApp { state })
        }),
    );
}

struct OdfizApp {
    state: Arc<Mutex<AppState>>,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("ODFIZ MONITOR");
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(30.0);

                if let Ok(mut data) = self.state.try_lock() {
                    // Monitor Status
                    let color = if data.is_active { egui::Color32::GREEN } else { egui::Color32::RED };
                    let status_text = if data.is_active { "SYSTEM ACTIVE" } else { "SYSTEM IDLE" };

                    ui.group(|ui| {
                        ui.set_width(250.0);
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new(status_text).color(color).strong().size(20.0));
                        ui.label(format!("Incoming Packets: {}", data.counter));
                        ui.add_space(10.0);
                    });

                    ui.add_space(40.0);

                    // Toggle Button
                    let btn_label = if data.is_active { "🛑 STOP MONITOR" } else { "▶️ START MONITOR" };
                    let btn_color = if data.is_active { egui::Color32::from_rgb(150, 0, 0) } else { egui::Color32::from_rgb(0, 100, 0) };

                    if ui.add_sized([200.0, 60.0], egui::Button::new(egui::RichText::new(btn_label).size(18.0)).fill(btn_color)).clicked() {
                        data.is_active = !data.is_active;
                        ctx.request_repaint();
                    }

                    // Power Saver Logic
                    if data.is_active {
                        ctx.request_repaint_after(Duration::from_secs(1));
                    }
                }
            });
        });
    }
}

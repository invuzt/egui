#![cfg(target_os = "android")]
use eframe::egui;
use egui_cable::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

struct AppState {
    is_internet_online: bool,
    counter: u64,
    connections: Vec<(usize, usize, usize)>,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let state = Arc::new(Mutex::new(AppState {
        is_internet_online: false,
        counter: 0,
        connections: Vec::new(),
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
                        if data.is_internet_online {
                            data.counter += 1;
                            "Signal Received"
                        } else {
                            "ERROR: NO CONNECTION"
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
        "Odfiz Core",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.4);
            Box::new(OdfizApp { state })
        }),
    );
}

// --- CUSTOM CABLE WIDGET DENGAN TOMBOL SILANG & WARNA DINAMIS ---
struct OdfizCable;

impl egui::Widget for OdfizCable {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let params = CableParams::get(ui);
        let mut bezier = params.bezier;
        
        // Tentukan warna: Hijau jika connect ke socket, Abu-abu jika lepas
        let is_connected = params.out_plug.is_connected();
        let color = if is_connected {
            egui::Color32::from_rgb(0, 255, 150) // Hijau Neon
        } else {
            egui::Color32::from_rgb(100, 100, 100) // Abu-abu
        };

        bezier.stroke = egui::Stroke::new(4.0, color);
        ui.painter().add(bezier);

        // Jika kabel disentuh/hover, munculkan tombol silang di tengah kabel
        let mut response = ui.add(params.cable_control);
        
        if response.hovered() || response.has_focus() {
            let mid_point = bezier.sample(0.5);
            let rect = egui::Rect::from_center_size(mid_point, egui::vec2(30.0, 30.0));
            
            // Tombol silang manual
            if ui.put(rect, egui::Button::new(egui::RichText::new("❌").size(14.0)).fill(egui::Color32::RED)).clicked() {
                response.mark_changed(); // Trigger disconnect via click
            }
        }
        
        response
    }
}

struct OdfizApp {
    state: Arc<Mutex<AppState>>,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ NETWORK CORE");
                ui.label("Pencet kabel untuk muncul tombol ❌");
                ui.separator();
            });

            if let Ok(mut data) = self.state.try_lock() {
                ui.columns(3, |cols| {
                    cols[0].vertical_centered(|ui| { ui.label("SOURCE"); ui.add(Port::new(10usize)); });
                    cols[1].vertical_centered(|ui| { ui.label("SERVER"); ui.add(Port::new(20usize)); });
                    cols[2].vertical_centered(|ui| { ui.label("MONITOR"); ui.add(Port::new(30usize)); });
                });

                ui.add_space(40.0);

                if data.connections.is_empty() {
                    let mut res = ui.add(Cable::new(0, Plug::to(10usize), Plug::unplugged()).widget(OdfizCable));
                    if let Some(p_id) = res.out_plug().connected_to() {
                        let target = *p_id.downcast_ref::<usize>().unwrap();
                        data.connections.push((0, 10, target));
                    }
                }

                let mut should_disconnect = false;
                for (id, a, b) in data.connections.iter() {
                    let mut res = ui.add(Cable::new(*id, Plug::to(*a), Plug::to(*b)).widget(OdfizCable));
                    
                    // Jika tombol silang di CustomCable diklik (mark_changed) atau kabel ditarik
                    if res.changed() || res.clicked() || res.out_plug().disconnected() {
                        should_disconnect = true;
                    }
                }

                if should_disconnect {
                    data.connections.clear();
                }

                data.is_internet_online = data.connections.iter().any(|&(_, _, b)| b == 20 || b == 30);

                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    if data.is_internet_online {
                        ui.colored_label(egui::Color32::from_rgb(0, 255, 150), "📡 SIGNAL: STABLE");
                        ui.label(format!("Traffic: {} packets", data.counter));
                    } else {
                        ui.colored_label(egui::Color32::from_rgb(255, 80, 80), "🚫 SIGNAL: LOST");
                    }
                });
            }
        });
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

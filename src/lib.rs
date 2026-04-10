#![cfg(target_os = "android")]
use eframe::egui;
use egui_cable::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

struct AppState {
    is_internet_online: bool,
    counter: u64,
    connections: Vec<(usize, usize, usize)>,
    last_update: Instant,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let state = Arc::new(Mutex::new(AppState {
        is_internet_online: false,
        counter: 0,
        connections: Vec::new(),
        last_update: Instant::now(),
    }));

    // Server Axum berjalan di background
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
                            "OK"
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
        "Odfiz Power Saver",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.4);
            
            // OPTIMASI 1: Matikan animasi visual bawaan yang berat
            let mut visuals = egui::Visuals::dark();
            visuals.faint_bg_color = egui::Color32::TRANSPARENT;
            cc.egui_ctx.set_visuals(visuals);

            Box::new(OdfizApp { state })
        }),
    );
}

struct OdfizApp {
    state: Arc<Mutex<AppState>>,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("🔋 ODFIZ ULTRA SAVER");
                ui.label(egui::RichText::new("Reactive Mode Active").italics().size(10.0));
                ui.separator();
            });

            if let Ok(mut data) = self.state.try_lock() {
                // Layout Sockets
                ui.columns(3, |cols| {
                    cols[0].vertical_centered(|ui| { ui.label("SRC"); ui.add(Port::new(10usize)); });
                    cols[1].vertical_centered(|ui| { ui.label("SKT A"); ui.add(Port::new(20usize)); });
                    cols[2].vertical_centered(|ui| { ui.label("SKT B"); ui.add(Port::new(30usize)); });
                });

                ui.add_space(20.0);

                // Logika Kabel
                if data.connections.is_empty() {
                    let res = ui.add(Cable::new(0, Plug::to(10usize), Plug::unplugged()));
                    if let Some(p_id) = res.out_plug().connected_to() {
                        let target = *p_id.downcast_ref::<usize>().unwrap();
                        data.connections.push((0, 10, target));
                        ctx.request_repaint(); // Refresh HANYA saat ada koneksi baru
                    }
                }

                for (id, a, b) in data.connections.iter() {
                    ui.add(Cable::new(*id, Plug::to(*a), Plug::to(*b)));
                }

                ui.add_space(20.0);

                // Tombol Reset
                if !data.connections.is_empty() {
                    if ui.add_sized([ui.available_width(), 40.0], egui::Button::new("✂ RESET").fill(egui::Color32::from_rgb(100, 0, 0))).clicked() {
                        data.connections.clear();
                        ctx.request_repaint(); // Refresh HANYA saat tombol ditekan
                    }
                }

                data.is_internet_online = data.connections.iter().any(|&(_, _, b)| b == 20 || b == 30);

                // OPTIMASI 2: Kontrol Repaint Berdasarkan Status
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    if data.is_internet_online {
                        ui.colored_label(egui::Color32::GREEN, "📡 STATUS: CONNECTED");
                        ui.label(format!("Data Packets: {}", data.counter));
                        
                        // Render ulang hanya 2 detik sekali untuk update counter
                        ctx.request_repaint_after(Duration::from_secs(2));
                    } else {
                        ui.colored_label(egui::Color32::GRAY, "💤 STATUS: DORMANT");
                        ui.label("Idle - No CPU usage");
                        // OPTIMASI 3: Jangan panggil request_repaint sama sekali saat offline
                        // Aplikasi akan berhenti menggambar sampai layar disentuh
                    }
                });
            }
        });
    }
}

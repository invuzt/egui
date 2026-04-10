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
                            "Signal OK"
                        } else {
                            "NO INTERNET"
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

struct OdfizApp {
    state: Arc<Mutex<AppState>>,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("ODFIZ NETWORK CORE").strong());
                ui.separator();
            });

            if let Ok(mut data) = self.state.try_lock() {
                // Layout Port
                ui.columns(3, |cols| {
                    cols[0].vertical_centered(|ui| { ui.label("SOURCE"); ui.add(Port::new(10usize)); });
                    cols[1].vertical_centered(|ui| { ui.label("SOCKET A"); ui.add(Port::new(20usize)); });
                    cols[2].vertical_centered(|ui| { ui.label("SOCKET B"); ui.add(Port::new(30usize)); });
                });

                ui.add_space(30.0);

                // Manajemen Kabel (Tanpa Custom Widget yang error)
                if data.connections.is_empty() {
                    let mut res = ui.add(Cable::new(0, Plug::to(10usize), Plug::unplugged()));
                    if let Some(p_id) = res.out_plug().connected_to() {
                        let target = *p_id.downcast_ref::<usize>().unwrap();
                        data.connections.push((0, 10, target));
                    }
                }

                for (id, a, b) in data.connections.iter() {
                    // Kabel standar saja biar aman
                    ui.add(Cable::new(*id, Plug::to(*a), Plug::to(*b)));
                }

                ui.add_space(20.0);

                // TOMBOL RESET / PUTUS KABEL
                if !data.connections.is_empty() {
                    if ui.add_sized([ui.available_width(), 40.0], egui::Button::new("✂ PUTUSKAN KONEKSI").fill(egui::Color32::from_rgb(150, 0, 0))).clicked() {
                        data.connections.clear();
                    }
                }

                // Logika Internet Online
                data.is_internet_online = data.connections.iter().any(|&(_, _, b)| b == 20 || b == 30);

                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    if data.is_internet_online {
                        ui.colored_label(egui::Color32::GREEN, "📡 STATUS: ONLINE");
                        ui.label(format!("Traffic: {} pkts", data.counter));
                    } else {
                        ui.colored_label(egui::Color32::RED, "🚫 STATUS: OFFLINE");
                        ui.label("Sambungkan kabel dari SOURCE ke SOCKET.");
                    }
                });
            }
        });
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

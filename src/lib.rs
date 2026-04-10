#![cfg(target_os = "android")]
use eframe::egui;
use egui_cable::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

struct AppState {
    status: String,
    counter: u64,
    connections: Vec<(usize, usize, usize)>,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let state = Arc::new(Mutex::new(AppState {
        status: "Disconnected".to_string(),
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
                        data.counter += 1;
                        "OK"
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
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("ODFIZ CORE");
                ui.separator();
            });

            if let Ok(mut data) = self.state.try_lock() {
                ui.columns(2, |cols| {
                    cols[0].vertical_centered(|ui| {
                        ui.label("SERVER");
                        ui.add(Port::new(100usize));
                    });
                    cols[1].vertical_centered(|ui| {
                        ui.label("MONITOR");
                        ui.add(Port::new(200usize));
                    });
                });

                ui.add_space(30.0);

                // FIX 1: Tambahkan 'mut' pada res
                if data.connections.is_empty() {
                    let mut res = ui.add(Cable::new(0, Plug::to(100usize), Plug::unplugged()));
                    if let Some(p_id) = res.out_plug().connected_to() {
                        if *p_id.downcast_ref::<usize>().unwrap() == 200 {
                            data.connections.push((0, 100, 200));
                            data.status = "LINK ACTIVE".to_string();
                        }
                    }
                }

                // FIX 2: Gunakan flag agar tidak mutasi data di dalam loop iterasi
                let mut should_disconnect = false;
                for (id, a, b) in data.connections.iter() {
                    let mut res = ui.add(Cable::new(*id, Plug::to(*a), Plug::to(*b)));
                    if res.out_plug().disconnected() {
                        should_disconnect = true;
                    }
                }

                if should_disconnect {
                    data.connections.clear();
                    data.status = "Disconnected".to_string();
                }

                ui.add_space(20.0);
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    if !data.connections.is_empty() {
                        ui.label(format!("STATUS: {}", data.status));
                        ui.label(format!("PULSE HITS: {}", data.counter));
                    } else {
                        ui.colored_label(egui::Color32::LIGHT_RED, "⚠ DATA LINK SEVERED");
                    }
                });
            }
        });
        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    }
}

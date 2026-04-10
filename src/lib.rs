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

    // Server Axum di Background
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
                        "Odfiz Core Pulse Received"
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
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(OdfizApp { state })
        }),
    );
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let font_data = include_bytes!("../assets/font.ttf");
    fonts.font_data.insert("jb".to_owned(), egui::FontData::from_static(font_data));
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "jb".to_owned());
    ctx.set_fonts(fonts);
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
                ui.heading(egui::RichText::new("ODFIZ CORE").strong().extra_letter_spacing(1.2));
                ui.label("Modular JNI Bridge");
                ui.separator();
            });

            if let Ok(mut data) = self.state.try_lock() {
                // Layout Port menggunakan columns
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

                // Manajemen Kabel
                if data.connections.is_empty() {
                    let res = ui.add(Cable::new(0, Plug::to(100usize), Plug::unplugged()));
                    if let Some(p_id) = res.out_plug().connected_to() {
                        if *p_id.downcast_ref::<usize>().unwrap() == 200 {
                            data.connections.push((0, 100, 200));
                            data.status = "LINK ACTIVE".to_string();
                        }
                    }
                }

                for (id, a, b) in data.connections.iter() {
                    let mut res = ui.add(Cable::new(*id, Plug::to(*a), Plug::to(*b)));
                    if res.out_plug().disconnected() {
                        data.connections.clear();
                        data.status = "Disconnected".to_string();
                    }
                }

                // Monitoring Panel
                ui.add_space(20.0);
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    if !data.connections.is_empty() {
                        ui.label(format!("STATUS: {}", data.status));
                        ui.label(format!("PULSE HITS: {}", data.counter));
                    } else {
                        ui.colored_label(egui::Color32::from_rgb(200, 50, 50), "⚠ DATA LINK SEVERED");
                        ui.label("Connect Server to Monitor to view stats.");
                    }
                });
            }
        });
        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    }
}

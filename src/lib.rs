#![cfg(target_os = "android")]
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

// State aplikasi untuk sinkronisasi Server & UI
struct AppState {
    status: String,
    counter: u64,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    
    // Inisialisasi State yang bisa dibagi antar thread
    let state = Arc::new(Mutex::new(AppState {
        status: "Starting...".to_string(),
        counter: 0,
    }));

    // --- SIKSAAN 1: Jalankan Server Axum di Background ---
    let server_state = state.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let app = axum::Router::new()
                .route("/", axum::routing::get(|| async { "Odfiz Core Server: High Performance Rust" }))
                .route("/hit", axum::routing::get({
                    let s = server_state.clone();
                    move || async move {
                        let mut data = s.lock().await;
                        data.counter += 1;
                        format!("Hit count: {}", data.counter)
                    }
                }));

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            {
                let mut data = server_state.lock().await;
                data.status = "Online (Port 3000)".to_string();
            }
            axum::serve(listener, app).await.unwrap();
        });
    });

    // --- UI EGUI (Frontend) ---
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Core",
        options,
        Box::new(|cc| {
            // DIET KETAT: Skala 1.4 agar terlihat seperti app sistem yang padat
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
        // Gunakan visual default yang bersih
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("ODFIZ CORE").strong().extra_letter_spacing(1.0));
                ui.separator();

                // Ambil data dari server thread (non-blocking)
                if let Ok(data) = self.state.try_lock() {
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label(format!("Server Status: {}", data.status));
                        ui.label(format!("API Hits: {}", data.counter));
                    });
                }

                ui.add_space(10.0);
                ui.label("Mesin Rust Aktif di Background");
                
                if ui.button("Simulasi Refresh").clicked() {
                    // Logic refresh
                }
            });
        });
        // Paksa refresh UI tiap detik untuk update status server
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}

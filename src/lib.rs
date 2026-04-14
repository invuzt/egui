#![cfg(target_os = "android")]
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

struct Node {
    pos: egui::Pos2,
    vel: egui::Vec2,
}

struct AppState {
    status: String,
    counter: u64,
    nodes: HashMap<String, Node>,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info)
    );

    let state = Arc::new(Mutex::new(AppState {
        status: "Starting...".to_string(),
        counter: 0,
        nodes: HashMap::new(),
    }));

    let server_state = state.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let app = axum::Router::new()
                .route("/", axum::routing::get(|| async { "Odfiz Graph API Active" }))
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
                data.status = "Graph & Server Online".to_string();
            }
            axum::serve(listener, app).await.unwrap();
        });
    });

    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Core",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.4);
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(OdfizApp { 
                state, 
                input_buffer: String::new() 
            })
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
    input_buffer: String,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        let dt = ctx.input(|i| i.stable_dt).min(0.1); 

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("ODFIZ GRAPH CORE").strong().extra_letter_spacing(1.0));
                
                // --- DISPLAY INPUT ---
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.label(egui::RichText::new(&self.input_buffer).size(24.0).color(egui::Color32::LIGHT_BLUE));
                    if self.input_buffer.is_empty() { ui.label("Pilih huruf..."); }
                });

                // --- VIRTUAL KEYBOARD (PILIHAN) ---
                ui.group(|ui| {
                    let keys = vec!["A", "B", "C", "RUST", "CORE", "ODFIZ", "1", "2"];
                    ui.horizontal_wrapped(|ui| {
                        for key in keys {
                            if ui.button(egui::RichText::new(key).size(18.0)).clicked() {
                                if let Ok(mut data) = self.state.try_lock() {
                                    let center = ui.max_rect().center();
                                    data.nodes.entry(key.to_string()).or_insert(Node {
                                        pos: center + egui::vec2(rand::random::<f32>() * 10.0, rand::random::<f32>() * 10.0),
                                        vel: egui::Vec2::ZERO,
                                    });
                                }
                            }
                        }
                        if ui.button(egui::RichText::new("❌ CLEAR").color(egui::Color32::RED)).clicked() {
                            if let Ok(mut data) = self.state.try_lock() {
                                data.nodes.clear();
                            }
                        }
                    });
                });

                ui.separator();

                // --- CANVAS GRAPH ---
                let (rect, _response) = ui.allocate_at_least(ui.available_size(), egui::Sense::hover());
                let painter = ui.painter_at(rect);
                let center = rect.center();

                if let Ok(mut data) = self.state.try_lock() {
                    let node_names: Vec<String> = data.nodes.keys().cloned().collect();
                    
                    for i in 0..node_names.len() {
                        for j in (i + 1)..node_names.len() {
                            let pos_i = data.nodes[&node_names[i]].pos;
                            let pos_j = data.nodes[&node_names[j]].pos;
                            let diff = pos_i - pos_j;
                            let dist_sq = diff.length_sq().max(100.0);
                            let force = diff / dist_sq * 2000.0;
                            
                            data.nodes.get_mut(&node_names[i]).unwrap().vel += force * dt;
                            data.nodes.get_mut(&node_names[j]).unwrap().vel -= force * dt;
                        }
                    }

                    for (name, node) in data.nodes.iter_mut() {
                        let to_center = center - node.pos;
                        node.vel += to_center * 1.5 * dt;
                        node.vel *= 0.92;
                        node.pos += node.vel * dt;

                        painter.line_segment([node.pos, center], egui::Stroke::new(1.0, egui::Color32::from_gray(50)));
                        painter.circle_filled(node.pos, 10.0, egui::Color32::from_rgb(255, 77, 109));
                        painter.text(node.pos + egui::vec2(0.0, 18.0), egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(14.0), egui::Color32::WHITE);
                    }
                    
                    let status_text = format!("Nodes: {} | Hits: {}", data.nodes.len(), data.counter);
                    painter.text(rect.left_bottom() + egui::vec2(10.0, -10.0), egui::Align2::LEFT_BOTTOM, status_text, egui::FontId::proportional(12.0), egui::Color32::LIGHT_GRAY);
                }
            });
        });
        ctx.request_repaint(); 
    }
}

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
    nodes: HashMap<String, Node>,
    // Relasi: "A" terhubung ke ["B", "C"]
    links: Vec<(String, String)>,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let state = Arc::new(Mutex::new(AppState {
        nodes: HashMap::new(),
        links: Vec::new(),
    }));

    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native("Odfiz Core", options, Box::new(move |cc| {
        cc.egui_ctx.set_pixels_per_point(1.4);
        setup_custom_fonts(&cc.egui_ctx);
        Box::new(OdfizApp { state })
    }));
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("jb".to_owned(), egui::FontData::from_static(include_bytes!("../assets/font.ttf")));
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "jb".to_owned());
    ctx.set_fonts(fonts);
}

struct OdfizApp {
    state: Arc<Mutex<AppState>>,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        let dt = 0.016;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(50.0);
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ LINKED GRAPH");

                // --- KEYBOARD PILIHAN (DENGAN RELASI) ---
                ui.horizontal_wrapped(|ui| {
                    if ui.button("➕ ADD CORE").clicked() {
                        self.add_node_with_link("CORE", None);
                    }
                    if ui.button("🌿 ADD RUST").clicked() {
                        self.add_node_with_link("RUST", Some("CORE"));
                    }
                    if ui.button("🦀 ADD ODFIZ").clicked() {
                        self.add_node_with_link("ODFIZ", Some("CORE"));
                    }
                    if ui.button("🔗 LINK RUST-ODFIZ").clicked() {
                        if let Ok(mut data) = self.state.try_lock() {
                            data.links.push(("RUST".to_string(), "ODFIZ".to_string()));
                        }
                    }
                    if ui.button("🗑 RESET").clicked() {
                        if let Ok(mut data) = self.state.try_lock() {
                            data.nodes.clear();
                            data.links.clear();
                        }
                    }
                });

                ui.separator();
                let (rect, _response) = ui.allocate_at_least(ui.available_size(), egui::Sense::hover());
                let painter = ui.painter_at(rect);
                let center = rect.center();

                if let Ok(mut data) = self.state.try_lock() {
                    let node_names: Vec<String> = data.nodes.keys().cloned().collect();

                    // 1. Repulsion: Semua saling dorong biar nggak numpuk
                    for i in 0..node_names.len() {
                        for j in (i + 1)..node_names.len() {
                            let pos_i = data.nodes[&node_names[i]].pos;
                            let pos_j = data.nodes[&node_names[j]].pos;
                            let diff = pos_i - pos_j;
                            let dist_sq = diff.length_sq().max(1000.0);
                            let force = diff / dist_sq * 4000.0;
                            data.nodes.get_mut(&node_names[i]).unwrap().vel += force * dt;
                            data.nodes.get_mut(&node_names[j]).unwrap().vel -= force * dt;
                        }
                    }

                    // 2. Spring Tension: Yang terhubung saling tarik
                    for (from, to) in &data.links {
                        if data.nodes.contains_key(from) && data.nodes.contains_key(to) {
                            let pos_from = data.nodes[from].pos;
                            let pos_to = data.nodes[to].pos;
                            let diff = pos_to - pos_from;
                            let dist = diff.length().max(1.0);
                            let desired_dist = 100.0; // Panjang "karet" penghubung
                            let spring_force = diff * (dist - desired_dist) * 0.05;
                            
                            data.nodes.get_mut(from).unwrap().vel += spring_force * dt;
                            data.nodes.get_mut(to).unwrap().vel -= spring_force * dt;
                            
                            // Gambar Garis Hubungan
                            painter.line_segment([pos_from, pos_to], egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 255, 150)));
                        }
                    }

                    // 3. Update & Brownian
                    for (name, node) in data.nodes.iter_mut() {
                        node.vel += egui::vec2((rand::random::<f32>() - 0.5) * 5.0, (rand::random::<f32>() - 0.5) * 5.0) * dt;
                        node.vel += (center - node.pos) * 0.5 * dt; // Gravity lemah
                        node.vel *= 0.92;
                        node.pos += node.vel;

                        painter.circle_filled(node.pos, 16.0, egui::Color32::from_rgb(255, 77, 109));
                        painter.text(node.pos, egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(12.0), egui::Color32::WHITE);
                    }
                }
            });
        });
        ctx.request_repaint();
    }
}

impl OdfizApp {
    fn add_node_with_link(&self, name: &str, parent: Option<&str>) {
        if let Ok(mut data) = self.state.try_lock() {
            let name_s = name.to_string();
            if !data.nodes.contains_key(&name_s) {
                data.nodes.insert(name_s.clone(), Node {
                    pos: egui::pos2(rand::random::<f32>() * 300.0, rand::random::<f32>() * 500.0),
                    vel: egui::Vec2::ZERO,
                });
            }
            if let Some(p) = parent {
                data.links.push((p.to_string(), name_s));
            }
        }
    }
}

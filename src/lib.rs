#![cfg(target_os = "android")]
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::time::Instant;
use std::fs;

struct Node {
    pos: egui::Pos2,
    vel: egui::Vec2,
    last_pressed: Option<Instant>,
}

struct AppState {
    status: String,
    counter: u64,
    nodes: HashMap<String, Node>,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let state = Arc::new(Mutex::new(AppState {
        status: "Ready".to_string(),
        counter: 0,
        nodes: HashMap::new(),
    }));

    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native("Odfiz Core", options, Box::new(|cc| {
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
            ui.add_space(45.0);
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ GRAPH CORE");

                ui.horizontal(|ui| {
                    if ui.button("📥 IMPORT DOWNLOAD/ODFIZ.TXT").clicked() {
                        // PATH Android Standard
                        let path = "/sdcard/Download/odfiz.txt";
                        match fs::read_to_string(path) {
                            Ok(content) => {
                                if let Ok(mut data) = self.state.try_lock() {
                                    for line in content.lines() {
                                        let word = line.trim().to_string();
                                        if !word.is_empty() {
                                            let center = ui.max_rect().center();
                                            data.nodes.entry(word).or_insert(Node {
                                                pos: center + egui::vec2(rand::random::<f32>() * 50.0, rand::random::<f32>() * 50.0),
                                                vel: egui::vec2(rand::random::<f32>(), rand::random::<f32>()),
                                                last_pressed: None,
                                            });
                                        }
                                    }
                                    data.status = format!("Loaded {} lines", content.lines().count());
                                }
                            }
                            Err(e) => {
                                if let Ok(mut data) = self.state.try_lock() {
                                    data.status = format!("Error: {}", e);
                                }
                            }
                        }
                    }
                    if ui.button("🗑 RESET").clicked() {
                        if let Ok(mut data) = self.state.try_lock() { data.nodes.clear(); }
                    }
                });

                ui.separator();

                let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::click_and_drag());
                let painter = ui.painter_at(rect);
                let center = rect.center();

                if let Ok(mut data) = self.state.try_lock() {
                    let mut to_remove = Vec::new();
                    let node_names: Vec<String> = data.nodes.keys().cloned().collect();
                    let mouse_pos = response.interact_pointer_pos();

                    for i in 0..node_names.len() {
                        for j in (i + 1)..node_names.len() {
                            let pos_i = data.nodes[&node_names[i]].pos;
                            let pos_j = data.nodes[&node_names[j]].pos;
                            let diff = pos_i - pos_j;
                            let dist_sq = diff.length_sq().max(900.0);
                            let force = diff / dist_sq * 3000.0;
                            data.nodes.get_mut(&node_names[i]).unwrap().vel += force * dt;
                            data.nodes.get_mut(&node_names[j]).unwrap().vel -= force * dt;
                        }
                    }

                    for (name, node) in data.nodes.iter_mut() {
                        node.vel += egui::vec2((rand::random::<f32>() - 0.5) * 5.0, (rand::random::<f32>() - 0.5) * 5.0) * dt;
                        let to_center = center - node.pos;
                        node.vel += to_center * 1.8 * dt;

                        if let Some(mpos) = mouse_pos {
                            let dist = (node.pos - mpos).length();
                            if dist < 40.0 && ctx.input(|i| i.pointer.primary_down()) {
                                if node.last_pressed.is_none() {
                                    node.last_pressed = Some(Instant::now());
                                    node.vel += egui::vec2(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5) * 100.0;
                                } else if node.last_pressed.unwrap().elapsed().as_secs_f32() > 0.8 {
                                    to_remove.push(name.clone());
                                }
                            } else { node.last_pressed = None; }
                        } else { node.last_pressed = None; }

                        node.vel *= 0.94;
                        node.pos += node.vel;

                        painter.line_segment([node.pos, center], egui::Stroke::new(1.0, egui::Color32::from_gray(60)));
                        painter.circle_filled(node.pos, 14.0, egui::Color32::from_rgb(255, 77, 109));
                        painter.text(node.pos, egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(11.0), egui::Color32::WHITE);
                    }
                    for name in to_remove { data.nodes.remove(&name); }
                    painter.text(rect.left_bottom() + egui::vec2(10.0, -10.0), egui::Align2::LEFT_BOTTOM, &data.status, egui::FontId::proportional(12.0), egui::Color32::LIGHT_GRAY);
                }
            });
        });
        ctx.request_repaint();
    }
}

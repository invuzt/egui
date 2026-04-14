#![cfg(target_os = "android")]
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

struct Node {
    pos: egui::Pos2,
    vel: egui::Vec2,
}

struct Link {
    from: String,
    to: String,
    is_active: bool,
}

struct AppState {
    nodes: HashMap<String, Node>,
    links: Vec<Link>,
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
        Box::new(OdfizApp { state, drag_node: None })
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
    drag_node: Option<String>,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        let dt = 0.016;
        let time = ctx.input(|i| i.time);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(50.0);
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ NETWORK SIMULATOR");

                ui.horizontal_wrapped(|ui| {
                    if ui.button("🌐 ADD NODE").clicked() {
                        self.add_node("SERVER");
                    }
                    if ui.button("⚡ CONNECT ALL").clicked() {
                        if let Ok(mut data) = self.state.try_lock() {
                            for l in data.links.iter_mut() { l.is_active = true; }
                        }
                    }
                    if ui.button("💀 BREAK LINKS").clicked() {
                        if let Ok(mut data) = self.state.try_lock() {
                            for l in data.links.iter_mut() { l.is_active = false; }
                        }
                    }
                });

                ui.separator();

                // AREA INTERAKSI
                let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::click_and_drag());
                let painter = ui.painter_at(rect);
                let center = rect.center();
                let mouse_pos = response.interact_pointer_pos();

                if let Ok(mut data) = self.state.try_lock() {
                    let names: Vec<String> = data.nodes.keys().cloned().collect();

                    // --- LOGIKA DRAG & TOUCH ---
                    if response.drag_started() {
                        if let Some(pos) = mouse_pos {
                            for name in &names {
                                if (data.nodes[name].pos - pos).length() < 30.0 {
                                    self.drag_node = Some(name.clone());
                                    break;
                                }
                            }
                        }
                    }
                    if response.drag_released() { self.drag_node = None; }

                    if let Some(ref name) = self.drag_node {
                        if let Some(pos) = mouse_pos {
                            if let Some(node) = data.nodes.get_mut(name) {
                                node.pos = pos;
                                node.vel = egui::Vec2::ZERO;
                            }
                        }
                    }

                    // --- PHYSICS ---
                    for i in 0..names.len() {
                        for j in (i + 1)..names.len() {
                            let pos_i = data.nodes[&names[i]].pos;
                            let pos_j = data.nodes[&names[j]].pos;
                            let diff = pos_i - pos_j;
                            let dist_sq = diff.length_sq().max(1200.0);
                            data.nodes.get_mut(&names[i]).unwrap().vel += (diff / dist_sq * 4500.0) * dt;
                            data.nodes.get_mut(&names[j]).unwrap().vel -= (diff / dist_sq * 4500.0) * dt;
                        }
                    }

                    for link in &data.links {
                        if let (Some(n1), Some(n2)) = (data.nodes.get(&link.from), data.nodes.get(&link.to)) {
                            let diff = n2.pos - n1.pos;
                            let dist = diff.length().max(1.0);
                            
                            // Visual Garis
                            if link.is_active {
                                // Efek Glow: Warna berubah-ubah & ketebalan berdenyut
                                let pulse = (time * 5.0).sin() as f32 * 0.5 + 0.5;
                                let color = egui::Color32::from_rgb(0, 255, 200);
                                painter.line_segment([n1.pos, n2.pos], egui::Stroke::new(2.0 + pulse * 2.0, color.gamma_multiply(0.3 + pulse * 0.4)));
                                painter.line_segment([n1.pos, n2.pos], egui::Stroke::new(1.0, color));
                                
                                // Gaya tarik (Spring) hanya aktif jika kabel nyambung
                                let force = diff * (dist - 120.0) * 0.08;
                                if self.drag_node.as_ref() != Some(&link.from) {
                                    data.nodes.get_mut(&link.from).unwrap().vel += force * dt;
                                }
                                if self.drag_node.as_ref() != Some(&link.to) {
                                    data.nodes.get_mut(&link.to).unwrap().vel -= force * dt;
                                }
                            } else {
                                // Kabel Putus: Abu-abu & melar (tidak ada gaya tarik)
                                painter.line_segment([n1.pos, n2.pos], egui::Stroke::new(1.0, egui::Color32::from_gray(60)));
                            }
                        }
                    }

                    // --- RENDER NODES ---
                    for (name, node) in data.nodes.iter_mut() {
                        if self.drag_node.as_ref() != Some(name) {
                            node.vel += (center - node.pos) * 0.4 * dt;
                            node.vel *= 0.91;
                            node.pos += node.vel;
                        }
                        painter.circle_filled(node.pos, 18.0, egui::Color32::from_rgb(40, 40, 40));
                        painter.circle_stroke(node.pos, 18.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
                        painter.text(node.pos, egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(10.0), egui::Color32::WHITE);
                    }
                }
            });
        });
        ctx.request_repaint();
    }
}

impl OdfizApp {
    fn add_node(&self, prefix: &str) {
        if let Ok(mut data) = self.state.try_lock() {
            let id = data.nodes.len();
            let name = format!("{}-{}", prefix, id);
            data.nodes.insert(name.clone(), Node {
                pos: egui::pos2(rand::random::<f32>() * 300.0, 300.0),
                vel: egui::Vec2::ZERO,
            });
            // Hubungkan otomatis ke node sebelumnya jika ada
            if id > 0 {
                let prev = format!("{}-{}", prefix, id - 1);
                data.links.push(Link { from: prev, to: name, is_active: true });
            }
        }
    }
}

#![cfg(target_os = "android")]
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

struct Node {
    pos: egui::Pos2,
    vel: egui::Vec2,
    is_server: bool,
}

#[derive(Clone)]
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
                ui.heading("ODFIZ RT/RW NET SIMULATOR");

                ui.horizontal_wrapped(|ui| {
                    if ui.button("🖥 SET SERVER").clicked() {
                        self.reset_and_add_server();
                    }
                    if ui.button("📱 ADD CLIENT").clicked() {
                        self.add_client();
                    }
                    if ui.button("⚡ TOGGLE NET").clicked() {
                        if let Ok(mut data) = self.state.try_lock() {
                            for l in data.links.iter_mut() { l.is_active = !l.is_active; }
                        }
                    }
                });

                ui.separator();

                let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::click_and_drag());
                let painter = ui.painter_at(rect);
                let center = rect.center();
                let mouse_pos = response.interact_pointer_pos();

                if let Ok(mut data) = self.state.try_lock() {
                    let names: Vec<String> = data.nodes.keys().cloned().collect();

                    // --- INTERAKSI SENTUH ---
                    if response.drag_started() {
                        if let Some(pos) = mouse_pos {
                            for name in &names {
                                if (data.nodes[name].pos - pos).length() < 35.0 {
                                    self.drag_node = Some(name.clone());
                                    break;
                                }
                            }
                        }
                    }
                    if response.drag_stopped() { self.drag_node = None; }

                    if let Some(ref name) = self.drag_node {
                        if let Some(pos) = mouse_pos {
                            if let Some(node) = data.nodes.get_mut(name) {
                                node.pos = pos;
                                node.vel = egui::Vec2::ZERO;
                            }
                        }
                    }

                    // --- PHYSICS: TOLAKAN ANTAR CLIENT ---
                    for i in 0..names.len() {
                        for j in (i + 1)..names.len() {
                            let pos_i = data.nodes[&names[i]].pos;
                            let pos_j = data.nodes[&names[j]].pos;
                            let diff = pos_i - pos_j;
                            let dist_sq = diff.length_sq().max(1500.0);
                            let force = (diff / dist_sq * 5000.0) * dt;
                            data.nodes.get_mut(&names[i]).unwrap().vel += force;
                            data.nodes.get_mut(&names[j]).unwrap().vel -= force;
                        }
                    }

                    // --- PHYSICS: HUBUNGAN SERVER-CLIENT ---
                    let links_to_draw = data.links.clone();
                    for link in links_to_draw {
                        if let (Some(n1), Some(n2)) = (data.nodes.get(&link.from), data.nodes.get(&link.to)) {
                            let diff = n2.pos - n1.pos;
                            let dist = diff.length().max(1.0);
                            
                            if link.is_active {
                                // Animasi Glow & Aliran Data
                                let pulse = (time * 4.0).sin() as f32 * 0.5 + 0.5;
                                let flow = (time * 10.0).cos() as f32 * 5.0;
                                let color = egui::Color32::from_rgb(0, 200, 255);
                                
                                painter.line_segment([n1.pos, n2.pos], egui::Stroke::new(3.0 + pulse * 2.0, color.gamma_multiply(0.2)));
                                painter.line_segment([n1.pos, n2.pos], egui::Stroke::new(1.0, color));
                                
                                // Gaya pegas (Spring Force)
                                let spring = diff * (dist - 150.0) * 0.05;
                                if self.drag_node.as_ref() != Some(&link.from) {
                                    data.nodes.get_mut(&link.from).unwrap().vel += spring * dt;
                                }
                                if self.drag_node.as_ref() != Some(&link.to) {
                                    data.nodes.get_mut(&link.to).unwrap().vel -= spring * dt;
                                }
                            } else {
                                painter.line_segment([n1.pos, n2.pos], egui::Stroke::new(1.0, egui::Color32::from_gray(70)));
                            }
                        }
                    }

                    // --- UPDATE & RENDER NODES ---
                    for (name, node) in data.nodes.iter_mut() {
                        if self.drag_node.as_ref() != Some(name) {
                            // Gaya gravitasi ke tengah layar
                            node.vel += (center - node.pos) * 0.3 * dt;
                            node.vel *= 0.92; // Friction
                            node.pos += node.vel;
                        }

                        let node_color = if node.is_server { egui::Color32::from_rgb(255, 100, 0) } else { egui::Color32::from_rgb(40, 40, 40) };
                        let stroke_color = if node.is_server { egui::Color32::YELLOW } else { egui::Color32::WHITE };
                        
                        painter.circle_filled(node.pos, 20.0, node_color);
                        painter.circle_stroke(node.pos, 20.0, egui::Stroke::new(2.0, stroke_color));
                        painter.text(node.pos, egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(10.0), egui::Color32::WHITE);
                    }
                }
            });
        });
        ctx.request_repaint();
    }
}

impl OdfizApp {
    fn reset_and_add_server(&self) {
        if let Ok(mut data) = self.state.try_lock() {
            data.nodes.clear();
            data.links.clear();
            data.nodes.insert("SERVER-01".to_string(), Node {
                pos: egui::pos2(200.0, 400.0),
                vel: egui::Vec2::ZERO,
                is_server: true,
            });
        }
    }

    fn add_client(&self) {
        if let Ok(mut data) = self.state.try_lock() {
            let client_id = data.nodes.values().filter(|n| !n.is_server).count() + 1;
            let name = format!("USER-{:02}", client_id);
            
            // Spawn posisi random dekat server
            data.nodes.insert(name.clone(), Node {
                pos: egui::pos2(rand::random::<f32>() * 300.0, rand::random::<f32>() * 600.0),
                vel: egui::Vec2::ZERO,
                is_server: false,
            });

            // Otomatis konek ke server (Cari node yang is_server = true)
            let server_name = data.nodes.iter()
                .find(|(_, n)| n.is_server)
                .map(|(k, _)| k.clone());

            if let Some(srv) = server_name {
                data.links.push(Link { from: srv, to: name, is_active: true });
            }
        }
    }
}

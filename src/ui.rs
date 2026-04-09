use eframe::egui;
use egui::{epaint::PathShape, Color32, Pos2, Rect, Shape, Stroke, Vec2};
use egui_plot::{Line, Plot, PlotPoints};
use crate::state::{AppState, Connection, Node, NodeType, SharedState};

pub struct OdfizApp {
    pub state: SharedState,
    pub current_page: String,
    pub dragging_node: Option<u64>,
}

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        Self { 
            state, 
            current_page: "Graph Editor".to_string(),
            dragging_node: None,
        }
    }
}

// --- FUNGSI EASING MATH (MURNI RUST) ---
fn ease_in_out_sine(t: f32) -> f32 {
    -( (std::f32::consts::PI * t).cos() - 1.0 ) / 2.0
}

fn ease_out_bounce(mut t: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;
    if t < 1.0 / d1 { n1 * t * t }
    else if t < 2.0 / d1 { t -= 1.5 / d1; n1 * t * t + 0.75 }
    else if t < 2.5 / d1 { t -= 2.25 / d1; n1 * t * t + 0.9375 }
    else { t -= 2.625 / d1; n1 * t * t + 0.984375 }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut show_panel = true;
        let mut time = 0.0;
        if let Ok(data) = self.state.try_lock() {
            show_panel = data.show_panel;
            time = data.animation_time;
        }

        ctx.set_visuals(egui::Visuals::dark());

        // --- SIDEBAR ---
        if show_panel {
            egui::SidePanel::left("sidebar").default_width(120.0).resizable(false).show(ctx, |ui| {
                ui.add_space(45.0);
                ui.vertical_centered_justified(|ui| {
                    if ui.selectable_label(self.current_page == "Graph Editor", "🌿 Graph").clicked() { self.current_page = "Graph Editor".to_string(); }
                    ui.add_space(10.0);
                    if ui.selectable_label(self.current_page == "Preview", "📺 Preview").clicked() { self.current_page = "Preview".to_string(); }
                });
            });
        }

        // --- MAIN PANEL ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(45.0);
            
            ui.horizontal(|ui| {
                let toggle_icon = if show_panel { "◀" } else { "☰" };
                if ui.button(toggle_icon).clicked() {
                    if let Ok(mut data) = self.state.try_lock() { data.show_panel = !data.show_panel; }
                }
                ui.heading(format!("ODFIZ Motion - {}", self.current_page));
            });
            ui.separator();

            match self.current_page.as_str() {
                "Preview" => {
                    // --- HALAMAN PREVIEW ANIMASI ---
                    if let Ok(data) = self.state.try_lock() {
                        let eased_value = ease_in_out_sine(data.animation_time);
                        ui.group(|ui| {
                            ui.label("Animation Preview (SineInOut Rotation)");
                            ui.add_space(20.0);
                            
                            // Gambar kotak yang berputar sesuai nilai easing
                            let (response, painter) = ui.allocate_painter(Vec2::splat(150.0), egui::Sense::hover());
                            let rect = response.rect;
                            let center = rect.center();
                            let rotation = eased_value * std::f32::consts::PI * 2.0;
                            let square_size = 50.0;
                            
                            let p1 = center + Vec2::rotated(Vec2::new(-square_size, -square_size), rotation);
                            let p2 = center + Vec2::rotated(Vec2::new(square_size, -square_size), rotation);
                            let p3 = center + Vec2::rotated(Vec2::new(square_size, square_size), rotation);
                            let p4 = center + Vec2::rotated(Vec2::new(-square_size, square_size), rotation);
                            
                            painter.add(Shape::Path(PathShape::closed_line(
                                vec![p1, p2, p3, p4],
                                Stroke::new(2.0, Color32::from_rgb(0, 255, 150)),
                            )));
                        });
                    }
                },
                _ => {
                    // --- HALAMAN GRAPH EDITOR (MODULAR ENGINE) ---
                    let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
                    let to_screen = eframe::emath::RectTransform::from_to(
                        Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                        response.rect,
                    );

                    if let Ok(mut data) = self.state.try_lock() {
                        // 1. Gambar Kabel (Koneksi)
                        for conn in &data.connections {
                            if let (Some(from), Some(to)) = (data.nodes.iter().find(|n| n.id == conn.from_node),
                                                             data.nodes.iter().find(|n| n.id == conn.to_node)) {
                                let start = to_screen * (from.pos + Vec2::new(80.0, 30.0));
                                let end = to_screen * (to.pos + Vec2::new(0.0, 30.0));
                                let cp1 = start + Vec2::new(40.0, 0.0);
                                let cp2 = end - Vec2::new(40.0, 0.0);
                                painter.add(Shape::cubic_bezier(
                                    start, cp1, cp2, end,
                                    Stroke::new(2.0, Color32::from_rgb(0, 150, 255))
                                ));
                            }
                        }

                        // 2. Gambar Node (Interaktif)
                        let mut nodes_to_update = Vec::new();
                        for node in &mut data.nodes {
                            let node_rect_in_canvas = Rect::from_min_size(node.pos, Vec2::new(80.0, 60.0));
                            let node_rect_on_screen = to_screen * node_rect_in_canvas;
                            
                            // Logika Drag & Drop Node
                            let node_id = ui.make_persistent_id(node.id);
                            let response = ui.interact(node_rect_on_screen, node_id, egui::Sense::drag());
                            if response.dragged() {
                                node.pos += response.drag_delta();
                            }

                            // Gambar Kotak Node
                            painter.add(Shape::rect_filled(node_rect_on_screen, 5.0, Color32::from_rgb(30, 30, 30)));
                            painter.add(Shape::rect_stroke(node_rect_on_screen, 5.0, Stroke::new(1.0, Color32::GRAY)));
                            
                            // Gambar Teks & Nilai di dalam Node
                            painter.text(to_screen * (node.pos + Vec2::new(5.0, 15.0)), egui::Align2::LEFT_TOP, &node.name, egui::FontId::proportional(12.0), Color32::WHITE);
                            
                            // Khusus Node Easing, Gambar Preview Kurva
                            if let NodeType::EasingCurve(name) = &node.node_type {
                                let eased_val = ease_in_out_sine(data.animation_time);
                                painter.text(to_screen * (node.pos + Vec2::new(5.0, 35.0)), egui::Align2::LEFT_TOP, format!("Val: {:.2}", eased_val), egui::FontId::proportional(10.0), Color32::YELLOW);
                            }
                        }
                    }
                }
            }
        });
        
        // --- LOOP WAKTU GLOBAL ANIMASI ---
        if let Ok(mut data) = self.state.try_lock() {
            data.animation_time += 0.01;
            if data.animation_time > 1.0 { data.animation_time = 0.0; }
        }
        ctx.request_repaint();
    }
}

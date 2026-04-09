use eframe::egui;
use egui::{epaint::{PathShape, CubicBezierShape}, Color32, Pos2, Rect, Shape, Stroke, Vec2, Rot2};
use crate::state::{NodeType, SharedState};

pub struct OdfizApp {
    pub state: SharedState,
    pub current_page: String,
}

const TUI_CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const TUI_YELLOW: Color32 = Color32::from_rgb(255, 255, 0);

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        Self { 
            state, 
            current_page: "Graph Editor".to_string(),
        }
    }
}

// Helper rotasi manual agar tidak error lagi
fn rotate_vec(v: Vec2, angle: f32) -> Vec2 {
    Rot2::from_angle(angle) * v
}

fn ease_in_out_sine(t: f32) -> f32 {
    -( (std::f32::consts::PI * t).cos() - 1.0 ) / 2.0
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut show_panel = true;
        if let Ok(data) = self.state.try_lock() {
            show_panel = data.show_panel;
        }

        ctx.set_visuals(egui::Visuals::dark());

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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(45.0);
            
            ui.horizontal(|ui| {
                if ui.button(if show_panel { "◀" } else { "☰" }).clicked() {
                    if let Ok(mut data) = self.state.try_lock() { data.show_panel = !data.show_panel; }
                }
                ui.heading("ODFIZ Motion Engine");
            });
            ui.separator();

            if self.current_page == "Preview" {
                if let Ok(data) = self.state.try_lock() {
                    let eased_value = ease_in_out_sine(data.animation_time);
                    let (response, painter) = ui.allocate_painter(Vec2::splat(200.0), egui::Sense::hover());
                    let center = response.rect.center();
                    let rotation = eased_value * std::f32::consts::PI * 2.0;
                    let sz = 50.0;
                    
                    // Menggunakan helper rotate_vec yang sudah pasti aman
                    let p1 = center + rotate_vec(Vec2::new(-sz, -sz), rotation);
                    let p2 = center + rotate_vec(Vec2::new(sz, -sz), rotation);
                    let p3 = center + rotate_vec(Vec2::new(sz, sz), rotation);
                    let p4 = center + rotate_vec(Vec2::new(-sz, sz), rotation);
                    
                    painter.add(Shape::Path(PathShape::closed_line(
                        vec![p1, p2, p3, p4],
                        Stroke::new(2.0, Color32::GREEN),
                    )));
                }
            } else {
                // --- GRAPH EDITOR ---
                let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
                let to_screen = eframe::emath::RectTransform::from_to(
                    Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                    response.rect,
                );

                if let Ok(mut data) = self.state.try_lock() {
                    // Gambar Kabel (CubicBezier PascalCase)
                    for conn in &data.connections {
                        if let (Some(f), Some(t)) = (data.nodes.iter().find(|n| n.id == conn.from_node),
                                                         data.nodes.iter().find(|n| n.id == conn.to_node)) {
                            let p0 = to_screen * (f.pos + Vec2::new(80.0, 30.0));
                            let p3 = to_screen * (t.pos + Vec2::new(0.0, 30.0));
                            let p1 = p0 + Vec2::new(40.0, 0.0);
                            let p2 = p3 - Vec2::new(40.0, 0.0);
                            painter.add(Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                                [p0, p1, p2, p3], false, Color32::TRANSPARENT, Stroke::new(2.0, TUI_CYAN)
                            )));
                        }
                    }

                    // Gambar Node
                    for node in &mut data.nodes {
                        let node_rect = Rect::from_min_size(node.pos, Vec2::new(80.0, 60.0));
                        let screen_rect = Rect::from_min_max(to_screen * node_rect.min, to_screen * node_rect.max);
                        
                        let node_id = ui.make_persistent_id(node.id);
                        let res = ui.interact(screen_rect, node_id, egui::Sense::drag());
                        if res.dragged() { node.pos += res.drag_delta(); }

                        painter.add(Shape::rect_filled(screen_rect, 5.0, Color32::from_rgb(30, 30, 30)));
                        painter.add(Shape::rect_stroke(screen_rect, 5.0, Stroke::new(1.0, Color32::GRAY)));
                        painter.text(screen_rect.min + Vec2::new(5.0, 5.0), egui::Align2::LEFT_TOP, &node.name, egui::FontId::proportional(12.0), Color32::WHITE);
                        
                        if let NodeType::EasingCurve(_) = node.node_type {
                            let val = ease_in_out_sine(data.animation_time);
                            painter.text(screen_rect.min + Vec2::new(5.0, 30.0), egui::Align2::LEFT_TOP, format!("{:.2}", val), egui::FontId::monospace(12.0), TUI_YELLOW);
                        }
                    }
                }
            }
        });

        if let Ok(mut data) = self.state.try_lock() {
            data.animation_time = (data.animation_time + 0.01) % 1.0;
        }
        ctx.request_repaint();
    }
}

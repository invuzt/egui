use eframe::egui;
use egui::{Color32, Pos2, Rect, Shape, Stroke, Vec2, epaint::CubicBezierShape};
use crate::state::SharedState;
use crate::mods::ModRegistry;

pub struct OdfizApp {
    pub state: SharedState,
    pub registry: ModRegistry,
}

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        Self { state, registry: ModRegistry::new() }
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        // --- 1. VIEWER PANEL (Bagian Atas) ---
        egui::TopBottomPanel::top("viewer").default_height(200.0).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("📺 MOTION PREVIEW").strong());
                
                let (response, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), 150.0), egui::Sense::hover());
                let center = response.rect.center();
                
                if let Ok(data) = self.state.try_lock() {
                    if let Some(last_node) = data.active_nodes.last() {
                        let m = &self.registry.available[last_node.mod_index];
                        m.draw_preview(&painter, center, last_node.current_value);
                    }
                }
            });
        });

        // --- 2. MOD LIBRARY (Floating/Sidebar bawah) ---
        egui::SidePanel::left("library").default_width(120.0).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.heading("📦 MODS");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, m) in self.registry.available.iter().enumerate() {
                    if ui.button(format!("+ {}", m.name())).clicked() {
                        if let Ok(mut data) = self.state.try_lock() {
                            let id = data.active_nodes.len() as u64;
                            data.active_nodes.push(crate::state::ActiveNode {
                                id, mod_index: i, pos: Pos2::new(100.0, 100.0), current_value: 0.0,
                            });
                            // Auto-connect ke node sebelumnya jika ada
                            if id > 0 {
                                data.connections.push(crate::state::Connection { from_node: id - 1, to_node: id });
                            }
                        }
                    }
                }
            });
        });

        // --- 3. NODE CANVAS (Bagian Tengah - Editor) ---
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
            
            if let Ok(mut data) = self.state.try_lock() {
                // Handle Zoom & Pan
                let zoom_delta = ctx.input(|i| i.zoom_delta());
                if zoom_delta != 1.0 { data.zoom *= zoom_delta; }
                if response.dragged_by(egui::PointerButton::Secondary) { data.pan += response.drag_delta(); }

                let to_screen = eframe::emath::RectTransform::from_to(
                    Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                    response.rect.translate(data.pan).scale_from_pivot(response.rect.center(), data.zoom),
                );

                // A. Gambar Kabel (Bezier)
                for conn in &data.connections {
                    if let (Some(f), Some(t)) = (data.nodes_iter(&data.active_nodes, conn.from_node),
                                                 data.nodes_iter(&data.active_nodes, conn.to_node)) {
                        let p_out = to_screen * (f.pos + Vec2::new(120.0, 25.0));
                        let p_in = to_screen * (t.pos + Vec2::new(0.0, 25.0));
                        painter.add(Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                            [p_out, p_out + Vec2::new(50.0 * data.zoom, 0.0), p_in - Vec2::new(50.0 * data.zoom, 0.0), p_in],
                            false, Color32::TRANSPARENT, Stroke::new(2.0 * data.zoom, Color32::from_rgb(0, 150, 255))
                        )));
                    }
                }

                // B. Gambar Nodes
                let time = data.animation_time;
                for node in &mut data.active_nodes {
                    let m = &self.registry.available[node.mod_index];
                    node.current_value = m.execute(time);

                    let node_rect = Rect::from_min_size(node.pos, Vec2::new(120.0, 50.0));
                    let screen_rect = Rect::from_min_max(to_screen * node_rect.min, to_screen * node_rect.max);
                    
                    let node_res = ui.interact(screen_rect, ui.make_persistent_id(node.id), egui::Sense::drag());
                    if node_res.dragged() { node.pos += node_res.drag_delta() / data.zoom; }

                    painter.add(Shape::rect_filled(screen_rect, 4.0, Color32::from_rgb(45, 45, 45)));
                    painter.add(Shape::rect_stroke(screen_rect, 4.0, Stroke::new(1.0, Color32::GRAY)));
                    painter.text(screen_rect.center(), egui::Align2::CENTER_CENTER, m.name(), egui::FontId::proportional(12.0 * data.zoom), Color32::WHITE);
                }

                data.animation_time = (data.animation_time + 0.005) % 1.0;
            }
        });
        ctx.request_repaint();
    }
}

// Helper untuk iterasi node
impl crate::state::AppState {
    fn nodes_iter<'a>(&self, nodes: &'a Vec<crate::state::ActiveNode>, id: u64) -> Option<&'a crate::state::ActiveNode> {
        nodes.iter().find(|n| n.id == id)
    }
}

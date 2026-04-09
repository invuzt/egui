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

        // --- VIEWER PANEL ---
        egui::TopBottomPanel::top("viewer").default_height(200.0).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("📺 MOTION PREVIEW").strong());
                let (res, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), 150.0), egui::Sense::hover());
                if let Ok(data) = self.state.try_lock() {
                    if let Some(last_node) = data.active_nodes.last() {
                        self.registry.available[last_node.mod_index].draw_preview(&painter, res.rect.center(), last_node.current_value);
                    }
                }
            });
        });

        // --- MOD LIBRARY ---
        egui::SidePanel::left("library").default_width(100.0).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.heading("📦 MODS");
            if ui.button("+ Circle").clicked() { self.add_node(0); }
            if ui.button("+ Square").clicked() { self.add_node(1); }
            if ui.button("+ Sine").clicked() { self.add_node(2); }
        });

        // --- EDITOR CANVAS ---
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
            
            if let Ok(mut data) = self.state.try_lock() {
                // Handle Zoom & Pan manual
                let zoom_delta = ctx.input(|i| i.zoom_delta());
                data.zoom *= zoom_delta;
                if response.dragged_by(egui::PointerButton::Secondary) { data.pan += response.drag_delta(); }

                // Perbaikan Transformasi Layar
                let base_rect = Rect::from_min_size(Pos2::ZERO, response.rect.size());
                let transformed_rect = response.rect.translate(data.pan);
                let to_screen = eframe::emath::RectTransform::from_to(base_rect, transformed_rect);

                // A. Kabel Bezier
                for conn in &data.connections {
                    let from = data.active_nodes.iter().find(|n| n.id == conn.from_node);
                    let to = data.active_nodes.iter().find(|n| n.id == conn.to_node);
                    if let (Some(f), Some(t)) = (from, to) {
                        let p_out = to_screen * (f.pos + Vec2::new(120.0, 25.0) * data.zoom);
                        let p_in = to_screen * (t.pos);
                        painter.add(Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                            [p_out, p_out + Vec2::new(50.0 * data.zoom, 0.0), p_in - Vec2::new(50.0 * data.zoom, 0.0), p_in],
                            false, Color32::TRANSPARENT, Stroke::new(2.0 * data.zoom, Color32::LIGHT_BLUE)
                        )));
                    }
                }

                // B. Nodes
                let time = data.animation_time;
                for node in &mut data.active_nodes {
                    node.current_value = self.registry.available[node.mod_index].execute(time);
                    let scaled_size = Vec2::new(120.0, 50.0) * data.zoom;
                    let screen_rect = Rect::from_min_size(to_screen * node.pos, scaled_size);
                    
                    let node_res = ui.interact(screen_rect, ui.make_persistent_id(node.id), egui::Sense::drag());
                    if node_res.dragged() { node.pos += node_res.drag_delta() / data.zoom; }

                    painter.add(Shape::rect_filled(screen_rect, 4.0, Color32::from_rgb(50, 50, 50)));
                    painter.text(screen_rect.center(), egui::Align2::CENTER_CENTER, 
                                 &self.registry.available[node.mod_index].name(), 
                                 egui::FontId::proportional(12.0 * data.zoom), Color32::WHITE);
                }
                data.animation_time = (data.animation_time + 0.005) % 1.0;
            }
        });
        ctx.request_repaint();
    }
}

impl OdfizApp {
    fn add_node(&mut self, mod_idx: usize) {
        if let Ok(mut data) = self.state.try_lock() {
            let id = data.active_nodes.len() as u64;
            data.active_nodes.push(crate::state::ActiveNode {
                id, mod_index: mod_idx, pos: Pos2::new(50.0, 50.0), current_value: 0.0,
            });
            if id > 0 { data.connections.push(crate::state::Connection { from_node: id-1, to_node: id }); }
        }
    }
}

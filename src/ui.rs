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

        // --- MOD LIBRARY & CONTROLS ---
        egui::SidePanel::left("library").default_width(110.0).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.heading("📦 MODS");
            ui.separator();
            if ui.button("➕ Circle").clicked() { self.add_node(0); }
            if ui.button("➕ Square").clicked() { self.add_node(1); }
            if ui.button("➕ Sine").clicked() { self.add_node(2); }
            
            ui.add_space(30.0);
            ui.heading("🛠 EDIT");
            ui.separator();
            if ui.button("⟲ Undo").clicked() { self.undo_action(); }
            if ui.button("🗑 Clear All").clicked() { self.clear_all(); }
        });

        // --- EDITOR CANVAS ---
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
            
            if let Ok(mut data) = self.state.try_lock() {
                let zoom_delta = ctx.input(|i| i.zoom_delta());
                data.zoom *= zoom_delta;
                if response.dragged_by(egui::PointerButton::Secondary) { data.pan += response.drag_delta(); }

                let current_zoom = data.zoom;
                let current_pan = data.pan;
                let time = data.animation_time;

                let to_screen = eframe::emath::RectTransform::from_to(
                    Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                    response.rect.translate(current_pan),
                );

                // A. Gambar Kabel
                for conn in &data.connections {
                    let from_node = data.active_nodes.iter().find(|n| n.id == conn.from_node);
                    let to_node = data.active_nodes.iter().find(|n| n.id == conn.to_node);
                    if let (Some(f), Some(t)) = (from_node, to_node) {
                        let p_out = to_screen * (f.pos + Vec2::new(120.0, 25.0) * current_zoom);
                        let p_in = to_screen * (t.pos);
                        painter.add(Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                            [p_out, p_out + Vec2::new(50.0 * current_zoom, 0.0), p_in - Vec2::new(50.0 * current_zoom, 0.0), p_in],
                            false, Color32::TRANSPARENT, Stroke::new(2.0 * current_zoom, Color32::LIGHT_BLUE)
                        )));
                    }
                }

                // B. Nodes dengan Tombol Delete
                let mut node_to_delete = None;
                for node in &mut data.active_nodes {
                    node.current_value = self.registry.available[node.mod_index].execute(time);
                    let scaled_size = Vec2::new(120.0, 50.0) * current_zoom;
                    let screen_rect = Rect::from_min_size(to_screen * node.pos, scaled_size);
                    
                    let node_res = ui.interact(screen_rect, ui.make_persistent_id(node.id), egui::Sense::drag());
                    if node_res.dragged() { node.pos += node_res.drag_delta() / current_zoom; }

                    painter.add(Shape::rect_filled(screen_rect, 4.0, Color32::from_rgb(45, 45, 45)));
                    
                    // Tombol Delete (X) di pojok kanan atas node
                    let del_rect = Rect::from_min_size(screen_rect.right_top() - Vec2::new(20.0, 0.0), Vec2::splat(20.0));
                    let del_res = ui.interact(del_rect, ui.make_persistent_id(format!("del_{}", node.id)), egui::Sense::click());
                    
                    painter.add(Shape::circle_filled(del_rect.center(), 8.0, if del_res.hovered() { Color32::RED } else { Color32::from_gray(80) }));
                    painter.text(del_rect.center(), egui::Align2::CENTER_CENTER, "×", egui::FontId::proportional(14.0), Color32::WHITE);

                    if del_res.clicked() { node_to_delete = Some(node.id); }

                    painter.text(screen_rect.center(), egui::Align2::CENTER_CENTER, 
                                 &self.registry.available[node.mod_index].name(), 
                                 egui::FontId::proportional(12.0 * current_zoom), Color32::WHITE);
                }

                // Eksekusi hapus jika ada yang diklik
                if let Some(id) = node_to_delete {
                    data.active_nodes.retain(|n| n.id != id);
                    data.connections.retain(|c| c.from_node != id && c.to_node != id);
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
            let id = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
            data.active_nodes.push(crate::state::ActiveNode {
                id, mod_index: mod_idx, pos: Pos2::new(50.0, 50.0), current_value: 0.0,
            });
            // Auto connect sederhana ke node terakhir
            if data.active_nodes.len() > 1 {
                let from = data.active_nodes[data.active_nodes.len() - 2].id;
                data.connections.push(crate::state::Connection { from_node: from, to_node: id });
            }
        }
    }

    fn undo_action(&mut self) {
        if let Ok(mut data) = self.state.try_lock() {
            data.active_nodes.pop();
            data.connections.pop();
        }
    }

    fn clear_all(&mut self) {
        if let Ok(mut data) = self.state.try_lock() {
            data.active_nodes.clear();
            data.connections.clear();
        }
    }
}

use eframe::egui;
use egui::{Color32, Rect, Shape, Stroke, Vec2, Pos2, epaint::CubicBezierShape};

pub fn show(ui: &mut egui::Ui, state: &crate::state::SharedState, registry: &crate::mods::ModRegistry) {
    let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
    
    if let Ok(mut data) = state.try_lock() {
        let zoom = data.zoom;
        let to_screen = eframe::emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect.translate(data.pan),
        );

        let mut node_to_delete = None;

        // 1. Gambar Kabel (Mendukung Cabang)
        for conn in &data.connections {
            if let (Some(f), Some(t)) = (data.active_nodes.iter().find(|n| n.id == conn.from_node),
                                         data.active_nodes.iter().find(|n| n.id == conn.to_node)) {
                let p_out = to_screen * (f.pos + Vec2::new(120.0, 25.0) * zoom);
                let p_in = to_screen * (t.pos);
                painter.add(Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                    [p_out, p_out + Vec2::new(50.0 * zoom, 0.0), p_in - Vec2::new(50.0 * zoom, 0.0), p_in],
                    false, Color32::TRANSPARENT, Stroke::new(2.0 * zoom, Color32::from_rgb(0, 150, 255))
                )));
            }
        }

        // 2. Gambar Nodes & Tombol Delete
        for node in &mut data.active_nodes {
            let screen_rect = Rect::from_min_size(to_screen * node.pos, Vec2::new(120.0, 50.0) * zoom);
            let node_res = ui.interact(screen_rect, ui.make_persistent_id(node.id), egui::Sense::drag());
            if node_res.dragged() { node.pos += node_res.drag_delta() / zoom; }

            painter.add(Shape::rect_filled(screen_rect, 4.0, Color32::from_rgb(45, 45, 45)));
            
            // Tombol Delete (X)
            let del_rect = Rect::from_min_size(screen_rect.right_top() - Vec2::new(20.0, 0.0), Vec2::splat(20.0));
            if ui.interact(del_rect, ui.make_persistent_id(format!("del_{}", node.id)), egui::Sense::click()).clicked() {
                node_to_delete = Some(node.id);
            }
            painter.text(del_rect.center(), egui::Align2::CENTER_CENTER, "×", egui::FontId::proportional(14.0), Color32::RED);

            painter.text(screen_rect.center(), egui::Align2::CENTER_CENTER, 
                         &registry.available[node.mod_index].name(), 
                         egui::FontId::proportional(12.0 * zoom), Color32::WHITE);
        }

        // Jalankan Hapus
        if let Some(id) = node_to_delete {
            data.active_nodes.retain(|n| n.id != id);
            data.connections.retain(|c| c.from_node != id && c.to_node != id);
        }
        
        // Sine Update (Kedut-kedut)
        data.animation_time = (ui.input(|i| i.time) as f32 % 1.0);
    }
}

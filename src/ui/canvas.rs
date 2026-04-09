use eframe::egui;
use egui::{Color32, Rect, Shape, Vec2, Pos2};

pub fn show(ui: &mut egui::Ui, state: &crate::state::SharedState, registry: &crate::mods::ModRegistry) {
    let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
    
    if let Ok(mut data) = state.try_lock() {
        let zoom = data.zoom;
        let to_screen = eframe::emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect.translate(data.pan),
        );

        let mut node_to_delete = None;
        let mut drag_ended_node = None;

        for i in 0..data.active_nodes.len() {
            let node = &mut data.active_nodes[i];
            let screen_rect = Rect::from_min_size(to_screen * node.pos, Vec2::new(130.0, 60.0) * zoom);
            
            let node_res = ui.interact(screen_rect, ui.make_persistent_id(node.id), egui::Sense::drag());
            
            if node_res.dragged() {
                node.pos += node_res.drag_delta() / zoom;
            }

            if node_res.drag_stopped() {
                drag_ended_node = Some(i);
            }

            // Visual: Jika punya parent, beri border khusus
            let color = if node.parent_id.is_some() { Color32::from_rgb(0, 200, 100) } else { Color32::from_rgb(45, 45, 45) };
            painter.add(Shape::rect_filled(screen_rect, 8.0, color));
            
            // Tombol Delete
            let del_rect = Rect::from_min_size(screen_rect.right_top() - Vec2::new(20.0, 0.0), Vec2::splat(20.0));
            if ui.interact(del_rect, ui.make_persistent_id(format!("del_{}", node.id)), egui::Sense::click()).clicked() {
                node_to_delete = Some(node.id);
            }
            
            painter.text(screen_rect.center(), egui::Align2::CENTER_CENTER, 
                         &registry.available[node.mod_index].name(), 
                         egui::FontId::proportional(12.0 * zoom), Color32::WHITE);
        }

        // Logika "Snap" saat drop
        if let Some(idx) = drag_ended_node {
            let dropped_pos = data.active_nodes[idx].pos;
            let dropped_id = data.active_nodes[idx].id;
            
            for j in 0..data.active_nodes.len() {
                if idx == j { continue; }
                let other_pos = data.active_nodes[j].pos;
                let other_id = data.active_nodes[j].id;

                if dropped_pos.distance(other_pos) < 50.0 {
                    data.active_nodes[idx].parent_id = Some(other_id);
                    data.active_nodes[idx].pos = other_pos + Vec2::new(10.0, 10.0); // Offset sedikit biar kelihatan menumpuk
                    break;
                }
            }
        }

        if let Some(id) = node_to_delete {
            data.active_nodes.retain(|n| n.id != id);
        }
        data.animation_time = (ui.input(|i| i.time) as f32 % 1.0);
    }
}

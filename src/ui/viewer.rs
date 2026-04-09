use eframe::egui;
pub fn show(ui: &mut egui::Ui, state: &crate::state::SharedState, registry: &crate::mods::ModRegistry) {
    ui.vertical_centered(|ui| {
        ui.add_space(10.0);
        ui.label(egui::RichText::new("📺 VIEWER").strong());
        let (res, painter) = ui.allocate_painter(egui::Vec2::new(ui.available_width(), 150.0), egui::Sense::hover());
        if let Ok(data) = state.try_lock() {
            if let Some(last_node) = data.active_nodes.last() {
                registry.available[last_node.mod_index].draw_preview(&painter, res.rect.center(), last_node.current_value);
            }
        }
    });
}

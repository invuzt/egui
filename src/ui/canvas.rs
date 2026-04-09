use eframe::egui;
use egui::{Color32, Rect, Shape, Stroke, Vec2, epaint::CubicBezierShape};

pub fn show(ui: &mut egui::Ui, state: &crate::state::SharedState, registry: &crate::mods::ModRegistry) {
    let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
    if let Ok(mut data) = state.try_lock() {
        let zoom = data.zoom;
        let to_screen = eframe::emath::RectTransform::from_to(
            Rect::from_min_size(egui::Pos2::ZERO, response.rect.size()),
            response.rect.translate(data.pan),
        );

        // Render Kabel & Nodes (Logika dipindah ke sini agar ui.rs bersih)
        // ... (Logika loop nodes sama seperti sebelumnya)
    }
}

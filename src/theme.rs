use eframe::egui;
use eframe::egui::{Color32, Frame, Ui, Margin, Stroke};

pub const COLOR_BG: Color32 = Color32::from_rgb(10, 10, 10);
pub const COLOR_ACCENT: Color32 = Color32::from_rgb(244, 63, 94); // Pink

pub fn odfiz_card(ui: &mut Ui, color: Color32, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none()
        .fill(Color32::from_rgb(25, 25, 25))
        .inner_margin(Margin::same(25.0))
        .rounding(30.0) // Sangat membulat
        .stroke(Stroke::new(1.5, color)) // Border warna-warni
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add_contents(ui);
        });
}

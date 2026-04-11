use eframe::egui;
use eframe::egui::{Color32, Frame, RichText, vec2, Ui, Response, Margin};

pub const COLOR_BG: Color32 = Color32::from_rgb(15, 15, 15);
pub const COLOR_CARD_OUTER: Color32 = Color32::from_rgb(25, 25, 25);
pub const COLOR_CARD_INNER: Color32 = Color32::from_rgb(35, 35, 35);
pub const COLOR_ACCENT: Color32 = Color32::from_rgb(244, 63, 94);
pub const COLOR_SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);

pub fn odfiz_card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none()
        .fill(COLOR_CARD_OUTER)
        .inner_margin(Margin::same(25.0)) // Padding diperbesar ke 25
        .rounding(24.0) // Corner lebih membulat (modern)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add_contents(ui);
        });
}

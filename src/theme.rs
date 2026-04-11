use eframe::egui;
use eframe::egui::{Color32, Frame, Ui, Margin};

pub const COLOR_BG: Color32 = Color32::from_rgb(15, 15, 15);
pub const COLOR_CARD: Color32 = Color32::from_rgb(25, 25, 25);
pub const COLOR_ACCENT: Color32 = Color32::from_rgb(244, 63, 94);

pub fn odfiz_card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none()
        .fill(COLOR_CARD)
        .inner_margin(Margin::same(25.0))
        .rounding(24.0)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add_contents(ui);
        });
}

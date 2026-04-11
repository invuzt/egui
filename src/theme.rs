use eframe::egui;
use eframe::egui::{Color32, Frame, RichText, vec2, Ui, Response};

pub const COLOR_BG: Color32 = Color32::from_rgb(15, 15, 15);
pub const COLOR_CARD_OUTER: Color32 = Color32::from_rgb(25, 25, 25);
pub const COLOR_CARD_INNER: Color32 = Color32::from_rgb(35, 35, 35);
pub const COLOR_ACCENT: Color32 = Color32::from_rgb(244, 63, 94);
pub const COLOR_TEXT_DIM: Color32 = Color32::from_rgb(120, 120, 120);

pub fn odfiz_card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none()
        .fill(COLOR_CARD_OUTER)
        .inner_margin(20.0)
        .rounding(18.0)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add_contents(ui);
        });
}

pub fn odfiz_button(ui: &mut Ui, text: &str) -> Response {
    let btn_text = RichText::new(text).color(Color32::WHITE).strong();
    ui.add(
        egui::Button::new(btn_text)
            .fill(COLOR_ACCENT)
            .min_size(vec2(ui.available_width(), 45.0))
            .rounding(10.0)
    )
}

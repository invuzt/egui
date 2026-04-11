use eframe::egui;
use eframe::egui::{Color32, Frame, Ui, Margin, Stroke, Rect, Vec2};

pub const COLOR_BG: Color32 = Color32::from_rgb(250, 250, 250);
pub const COLOR_CARD: Color32 = Color32::from_rgb(255, 255, 255);
pub const COLOR_ACCENT: Color32 = Color32::from_rgb(244, 63, 94);
pub const COLOR_TEXT: Color32 = Color32::from_rgb(30, 30, 30);

pub fn draw_grid_item(ui: &mut Ui, icon: &str, label: &str, color: Color32, active: bool) -> egui::Response {
    let stroke = if active { Stroke::new(2.5, color) } else { Stroke::new(1.0, Color32::from_rgb(230, 230, 230)) };
    
    let (rect, response) = ui.allocate_at_least(egui::vec2(ui.available_width() / 2.2, 90.0), egui::Sense::click());
    
    ui.painter().rect(rect, 20.0, COLOR_CARD, stroke);

    ui.allocate_ui_at_rect(rect, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(15.0);
            ui.label(egui::RichText::new(icon).size(28.0));
            ui.label(egui::RichText::new(label).size(14.0).strong().color(COLOR_TEXT));
        });
    });

    response
}

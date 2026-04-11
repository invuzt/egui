use eframe::egui;
use eframe::egui::{Color32, Frame, Ui, Margin, Stroke};

pub const COLOR_BG: Color32 = Color32::from_rgb(250, 250, 250); // Background Putih Bersih
pub const COLOR_CARD: Color32 = Color32::from_rgb(255, 255, 255); // Kartu Putih
pub const COLOR_ACCENT: Color32 = Color32::from_rgb(244, 63, 94); // Pink Odfiz
pub const COLOR_TEXT: Color32 = Color32::from_rgb(30, 30, 30);
pub const COLOR_BORDER: Color32 = Color32::from_rgb(230, 230, 230);

pub fn draw_search_bar(ui: &mut Ui) {
    Frame::none()
        .fill(Color32::from_rgb(245, 245, 245))
        .inner_margin(Margin::symmetric(20.0, 12.0))
        .rounding(25.0) // Oval
        .stroke(Stroke::new(1.0, COLOR_BORDER))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.label(egui::RichText::new("Search for services...").color(Color32::GRAY));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("⚙️");
                });
            });
        });
}

pub fn draw_grid_item(ui: &mut Ui, icon: &str, label: &str, active: bool) -> egui::Response {
    let stroke = if active { Stroke::new(2.0, COLOR_ACCENT) } else { Stroke::new(1.0, COLOR_BORDER) };
    
    let (rect, response) = ui.allocate_at_least(egui::vec2(ui.available_width() / 2.2, 80.0), egui::Sense::click());
    
    ui.painter().rect(
        rect, 
        15.0, 
        COLOR_CARD, 
        stroke
    );

    ui.allocate_ui_at_rect(rect, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(egui::RichText::new(icon).size(25.0));
            ui.label(egui::RichText::new(label).size(14.0).strong().color(COLOR_TEXT));
        });
    });

    response
}

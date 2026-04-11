use eframe::egui;
use eframe::egui::{Visuals, Margin};

pub fn apply_clean_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 12.0);
    style.spacing.window_margin = Margin::same(20.0);
    
    // Font yang nyaman di mata
    style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Proportional));
    style.text_styles.insert(egui::TextStyle::Button, egui::FontId::new(18.0, egui::FontFamily::Proportional));
    
    style.visuals = Visuals::dark();
    ctx.set_style(style);
}

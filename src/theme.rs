use eframe::egui;
use eframe::egui::{Color32, Visuals, Margin, Vec2};

pub fn apply_ios_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Font seimbang & bersih
    let font_id = egui::FontId::new(18.0, egui::FontFamily::Proportional);
    style.text_styles.insert(egui::TextStyle::Body, font_id.clone());
    style.text_styles.insert(egui::TextStyle::Button, font_id.clone());
    
    // iOS Light Mode Visuals
    style.visuals = Visuals::light();
    style.visuals.panel_fill = Color32::from_rgb(242, 242, 247); // Background iOS Gray
    
    // Styling Tombol khas iOS
    style.visuals.widgets.inactive.bg_fill = Color32::WHITE;
    style.visuals.widgets.inactive.rounding = 12.0;
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Color32::from_rgb(0, 122, 255)); // iOS Blue
    
    style.spacing.item_spacing = Vec2::new(10.0, 15.0);
    style.spacing.button_padding = Vec2::new(20.0, 12.0);
    
    ctx.set_style(style);
}

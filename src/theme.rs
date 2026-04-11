use eframe::egui;

pub fn apply_minimal_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    let font_id = egui::FontId::new(20.0, egui::FontFamily::Proportional);
    style.text_styles.insert(egui::TextStyle::Body, font_id.clone());
    style.text_styles.insert(egui::TextStyle::Button, font_id.clone());
    
    style.visuals = egui::Visuals::dark();
    ctx.set_style(style);
}

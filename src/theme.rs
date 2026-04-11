use eframe::egui;

pub fn apply_minimal_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Set semua font size rata 20.0 agar seimbang
    let font_id = egui::FontId::new(20.0, egui::FontFamily::Proportional);
    style.text_styles.insert(egui::TextStyle::Body, font_id.clone());
    style.text_styles.insert(egui::TextStyle::Button, font_id.clone());
    style.text_styles.insert(egui::TextStyle::Heading, font_id.clone());
    
    style.visuals = egui::Visuals::dark();
    style.spacing.item_spacing = egui::vec2(10.0, 20.0);
    
    ctx.set_style(style);
}

use eframe::egui;
use eframe::egui::{Color32, Visuals, Margin, Vec2, Stroke};

pub fn apply_global_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // 1. Spacing & Margin (Kunci Kerapihan)
    style.spacing.item_spacing = Vec2::new(10.0, 15.0); // Jarak antar elemen
    style.spacing.window_margin = Margin::same(15.0);
    style.spacing.button_padding = Vec2::new(15.0, 10.0);
    
    // 2. Rounding (Semua membulat konsisten)
    style.visuals.widgets.noninteractive.rounding = 25.0; // Rounding Kartu
    style.visuals.widgets.inactive.rounding = 15.0;      // Rounding Tombol
    style.visuals.widgets.hovered.rounding = 15.0;
    style.visuals.widgets.active.rounding = 15.0;

    // 3. Warna Default Text
    style.visuals.override_text_color = Some(Color32::WHITE);
    
    ctx.set_style(style);
}

pub fn draw_card(ui: &mut egui::Ui, bg_color: Color32, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(bg_color)
        .inner_margin(Margin::same(20.0))
        .rounding(25.0)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add_contents(ui);
        });
}

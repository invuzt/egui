use eframe::egui;
use eframe::egui::{Color32, Frame, Ui, Margin, Stroke, Pos2, Vec2};

pub const COLOR_BG: Color32 = Color32::from_rgb(10, 10, 10);
pub const COLOR_ACCENT: Color32 = Color32::from_rgb(244, 63, 94); // Re-added

pub fn odfiz_card(ui: &mut Ui, bg_color: Color32, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none()
        .fill(bg_color)
        .inner_margin(Margin::same(25.0))
        .rounding(32.0)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add_contents(ui);
        });
}

pub fn draw_hud_icon(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_at_least(Vec2::splat(size), egui::Sense::hover());
    let painter = ui.painter();
    let center = rect.center();
    let r = size / 2.0;
    painter.circle_stroke(center, r, Stroke::new(1.0, color.gamma_multiply(0.3)));
    painter.line_segment([Pos2::new(center.x - r, center.y), Pos2::new(center.x + r, center.y)], Stroke::new(1.0, color));
    painter.line_segment([Pos2::new(center.x, center.y - r), Pos2::new(center.x, center.y + r)], Stroke::new(1.0, color));
    painter.circle_filled(center, 2.0, color);
}

use eframe::egui;

pub fn apply_basic_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals::dark();
    ctx.set_style(style);
}

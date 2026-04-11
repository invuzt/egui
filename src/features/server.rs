use eframe::egui;
use eframe::egui::{Color32, RichText};
use crate::theme;

pub struct LiteServer {
    pub is_running: bool,
}

impl LiteServer {
    pub fn new() -> Self {
        Self { is_running: false }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Server Status:");
                let (txt, color) = if self.is_running { ("ON", Color32::GREEN) } else { ("OFF", theme::COLOR_ACCENT) };
                ui.label(RichText::new(txt).color(color).strong());
            });

            ui.add_space(15.0);

            let btn_label = if self.is_running { "STOP SERVER" } else { "RUN SERVER" };
            if ui.add(egui::Button::new(RichText::new(btn_label).size(18.0).strong())
                .fill(theme::COLOR_ACCENT)
                .min_size(egui::vec2(ui.available_width(), 45.0))
                .rounding(12.0)).clicked() {
                self.is_running = !self.is_running;
            }
        });
    }
}

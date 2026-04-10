use eframe::egui;
use super::OdfizModule;

pub struct CounterFeature {
    pub count: u64,
}

impl CounterFeature {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl OdfizModule for CounterFeature {
    fn name(&self) -> &str { "🔢 Penghitung Otomatis" }
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading(self.name());
            ui.label(format!("Nilai saat ini: {}", self.count));
            if ui.button("➕ Tambah Angka").clicked() {
                self.count += 1;
            }
        });
    }
}

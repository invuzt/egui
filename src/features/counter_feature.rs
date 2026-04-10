use eframe::egui;

pub struct CounterFeature {
    pub count: u64,
}

impl CounterFeature {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Modul Counter");
            ui.label(format!("Nilai: {}", self.count));
            if ui.button("Tambah").clicked() {
                self.count += 1;
            }
        });
    }
}

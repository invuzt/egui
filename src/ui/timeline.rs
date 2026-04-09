use eframe::egui;

pub fn show(ui: &mut egui::Ui, state: &crate::state::SharedState) {
    ui.vertical(|ui| {
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("🎞 Timeline:");
            if let Ok(mut data) = state.try_lock() {
                // Tombol Play/Pause (Simulasi)
                if ui.button("▶").clicked() { /* Logika resume */ }
                if ui.button("⏸").clicked() { /* Logika pause */ }
                
                // Slider untuk geser waktu (0.0 sampai 1.0)
                ui.add(egui::Slider::new(&mut data.animation_time, 0.0..=1.0)
                    .show_value(true)
                    .text("Progress"));
            }
        });
    });
}

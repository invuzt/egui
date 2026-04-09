use eframe::egui;
use crate::state::SharedState;

pub struct OdfizApp {
    pub state: SharedState,
}

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("ODFIZ CORE").strong().extra_letter_spacing(1.2));
                ui.separator();

                if let Ok(data) = self.state.try_lock() {
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label(format!("Server: {}", data.server_status));
                        ui.label(format!("API Hits: {}", data.api_hits));
                    });
                }

                ui.add_space(15.0);
                ui.label(egui::RichText::new("Rust Engine Active").weak().size(10.0));
                
                if ui.button("Manual Refresh").clicked() {
                    // Logic refresh bisa ditambahkan di sini
                }
            });
        });
        
        // Refresh otomatis setiap 500ms untuk update data dari server
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}

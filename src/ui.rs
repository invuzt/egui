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
        let mut show_panel = true;
        if let Ok(data) = self.state.try_lock() {
            show_panel = data.show_panel;
        }

        // --- SIDE PANEL (Bisa di-toggle) ---
        if show_panel {
            egui::SidePanel::left("menu_panel")
                .default_width(110.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add_space(15.0);
                    ui.heading("ODFIZ");
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.selectable_label(true, "🏠 Home");
                    ui.selectable_label(false, "⚙ Settings");
                    ui.selectable_label(false, "📝 Logs");
                    
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        if ui.button("HIDE").clicked() {
                           if let Ok(mut data) = self.state.try_lock() { data.show_panel = false; }
                        }
                        ui.add_space(10.0);
                    });
                });
        }

        // --- CENTRAL PANEL ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if !show_panel {
                    if ui.button("MENU").clicked() {
                        if let Ok(mut data) = self.state.try_lock() { data.show_panel = true; }
                    }
                }
                ui.heading("Dashboard Monitor");
            });
            ui.separator();

            if let Ok(data) = self.state.try_lock() {
                ui.label(format!("Status: {}", data.server_status));
                ui.label(format!("Total Hits: {}", data.api_hits));
                
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.label("Access Logs:");
                    for log in data.logs.iter().rev() {
                        ui.label(egui::RichText::new(format!("[{}] Access from: {}", log.time, log.ip)).monospace().size(10.0));
                    }
                });
            }
        });

        ctx.request_repaint();
    }
}

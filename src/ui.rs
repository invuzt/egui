use eframe::egui;
use crate::state::SharedState;

pub struct OdfizApp {
    pub state: SharedState,
    pub current_page: String, // Untuk navigasi antar menu
}

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        Self { 
            state,
            current_page: "Home".to_string(),
        }
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut is_dark = true;
        let mut show_panel = true;

        if let Ok(data) = self.state.try_lock() {
            is_dark = data.dark_mode;
            show_panel = data.show_panel;
        }

        // --- LOGIKA GANTI TEMA ---
        if is_dark {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // --- SIDE PANEL ---
        if show_panel {
            egui::SidePanel::left("menu_panel")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add_space(40.0); // Safe Area agar tidak kena Status Bar
                    ui.heading("ODFIZ");
                    ui.separator();
                    
                    if ui.selectable_label(self.current_page == "Home", "🏠 Home").clicked() {
                        self.current_page = "Home".to_string();
                    }
                    if ui.selectable_label(self.current_page == "Settings", "⚙ Settings").clicked() {
                        self.current_page = "Settings".to_string();
                    }
                    
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        if ui.button("HIDE").clicked() {
                           if let Ok(mut data) = self.state.try_lock() { data.show_panel = false; }
                        }
                        ui.add_space(20.0);
                    });
                });
        }

        // --- CENTRAL PANEL ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(40.0); // Safe Area Top
            
            ui.horizontal(|ui| {
                if !show_panel {
                    if ui.button("MENU").clicked() {
                        if let Ok(mut data) = self.state.try_lock() { data.show_panel = true; }
                    }
                }
                ui.heading(format!("{} Area", self.current_page));
            });
            ui.separator();

            match self.current_page.as_str() {
                "Settings" => {
                    ui.group(|ui| {
                        ui.label("Appearance");
                        if let Ok(mut data) = self.state.try_lock() {
                            if ui.radio_value(&mut data.dark_mode, true, "Dark Mode").clicked() ||
                               ui.radio_value(&mut data.dark_mode, false, "Light Mode").clicked() {
                                // Tema akan otomatis berubah di frame berikutnya
                            }
                        }
                    });
                },
                _ => { // Halaman Home (Default)
                    if let Ok(data) = self.state.try_lock() {
                        ui.label(format!("Server Status: {}", data.server_status));
                        ui.label(format!("Total Hits: {}", data.api_hits));
                        ui.add_space(10.0);
                        ui.label("Recent Logs:");
                        for log in data.logs.iter().rev().take(5) {
                            ui.label(egui::RichText::new(format!("[{}] {}", log.time, log.ip)).monospace().size(10.0));
                        }
                    }
                }
            }
        });

        ctx.request_repaint();
    }
}

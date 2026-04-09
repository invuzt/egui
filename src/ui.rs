use eframe::egui;
use crate::state::SharedState;

pub struct OdfizApp {
    pub state: SharedState,
    pub current_page: String,
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

        // Set Tema
        if is_dark {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // --- SIDE PANEL (Hanya muncul jika show_panel true) ---
        if show_panel {
            egui::SidePanel::left("menu_panel")
                .resizable(false)
                .default_width(70.0) // Makin ramping makin mantap
                .show(ctx, |ui| {
                    ui.add_space(45.0); 
                    ui.vertical_centered(|ui| {
                        // Navigasi Ikon
                        if ui.selectable_label(self.current_page == "Home", "🏠 Home").clicked() {
                            self.current_page = "Home".to_string();
                        }
                        ui.add_space(20.0);
                        if ui.selectable_label(self.current_page == "Settings", "⚙ Settings").clicked() {
                            self.current_page = "Settings".to_string();
                        }
                    });
                });
        }

        // --- CENTRAL PANEL ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(45.0); // Safe Area
            
            ui.horizontal(|ui| {
                // TOMBOL TOGGLE TUNGGAL
                let button_text = if show_panel { "◀" } else { "☰" };
                if ui.button(button_text).clicked() {
                    if let Ok(mut data) = self.state.try_lock() {
                        data.show_panel = !data.show_panel; // Balikkan status (Toggle)
                    }
                }
                
                ui.heading(&self.current_page);
            });
            ui.separator();

            // Konten Halaman
            match self.current_page.as_str() {
                "Settings" => {
                    ui.group(|ui| {
                        ui.label("Appearance");
                        if let Ok(mut data) = self.state.try_lock() {
                            ui.radio_value(&mut data.dark_mode, true, "Dark Mode");
                            ui.radio_value(&mut data.dark_mode, false, "Light Mode");
                        }
                    });
                },
                _ => {
                    if let Ok(data) = self.state.try_lock() {
                        ui.label(format!("Server: {}", data.server_status));
                        ui.label(format!("API Hits: {}", data.api_hits));
                        ui.add_space(20.0);
                        ui.label("Access Logs:");
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            for log in data.logs.iter().rev().take(5) {
                                ui.label(egui::RichText::new(format!("[{}] {}", log.time, log.ip)).monospace().size(10.0));
                            }
                        });
                    }
                }
            }
        });

        ctx.request_repaint();
    }
}

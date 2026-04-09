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

        if is_dark {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // --- SIDE PANEL (RAMPING: 80px) ---
        if show_panel {
            egui::SidePanel::left("menu_panel")
                .resizable(false)
                .default_width(80.0) // Lebih sempit
                .show(ctx, |ui| {
                    ui.add_space(45.0); // Safe area status bar
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("ODFIZ").strong().size(14.0));
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(15.0);
                        
                        // Menu Items (Teks lebih kecil agar muat)
                        if ui.selectable_label(self.current_page == "Home", "🏠").clicked() {
                            self.current_page = "Home".to_string();
                        }
                        ui.add_space(15.0);
                        if ui.selectable_label(self.current_page == "Settings", "⚙").clicked() {
                            self.current_page = "Settings".to_string();
                        }
                        
                        ui.add_space(30.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // TOMBOL HIDE: Sekarang di atas, mudah ditekan
                        if ui.button("HIDE").clicked() {
                           if let Ok(mut data) = self.state.try_lock() { data.show_panel = false; }
                        }
                    });
                });
        }

        // --- CENTRAL PANEL ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(45.0); // Safe area top
            
            ui.horizontal(|ui| {
                if !show_panel {
                    if ui.button("☰ MENU").clicked() {
                        if let Ok(mut data) = self.state.try_lock() { data.show_panel = true; }
                    }
                }
                ui.heading(&self.current_page);
            });
            ui.separator();

            match self.current_page.as_str() {
                "Settings" => {
                    ui.group(|ui| {
                        ui.label("Appearance");
                        if let Ok(mut data) = self.state.try_lock() {
                            ui.radio_value(&mut data.dark_mode, true, "Dark");
                            ui.radio_value(&mut data.dark_mode, false, "Light");
                        }
                    });
                },
                _ => {
                    if let Ok(data) = self.state.try_lock() {
                        ui.label(format!("Server: {}", data.server_status));
                        ui.label(format!("Hits: {}", data.api_hits));
                        ui.add_space(15.0);
                        ui.label("Access Logs:");
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            for log in data.logs.iter().rev().take(8) {
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

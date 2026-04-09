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

        // --- SIDE PANEL (Rata Kiri dengan Teks) ---
        if show_panel {
            egui::SidePanel::left("main_sidebar")
                .resizable(false)
                .default_width(120.0) // Sedikit lebih lebar untuk teks
                .show(ctx, |ui| {
                    ui.add_space(45.0); // Safe Area
                    
                    ui.vertical(|ui| {
                        ui.set_min_width(120.0);
                        ui.style_mut().spacing.button_padding = egui::vec2(8.0, 8.0);

                        // Menu Utama: Ikon + Teks
                        if ui.selectable_label(self.current_page == "Home", "🏠  Home").clicked() {
                            self.current_page = "Home".to_string();
                        }
                        
                        ui.add_space(8.0);
                        
                        if ui.selectable_label(self.current_page == "Settings", "⚙  Settings").clicked() {
                            self.current_page = "Settings".to_string();
                        }

                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // Menu ala bar (dropdown) di dalam Sidebar
                        ui.menu_button("📁  System", |ui| {
                            if ui.button("⟲  Restart").clicked() {
                                // Logic restart bisa di sini
                            }
                            if ui.button("❌  Quit").clicked() {
                                std::process::exit(0);
                            }
                        });
                    });
                });
        }

        // --- CENTRAL PANEL ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(45.0);
            
            ui.horizontal(|ui| {
                let toggle_icon = if show_panel { "◀" } else { "☰" };
                if ui.button(toggle_icon).clicked() {
                    if let Ok(mut data) = self.state.try_lock() {
                        data.show_panel = !data.show_panel;
                    }
                }
                ui.heading(format!("Dashboard: {}", self.current_page));
            });
            ui.separator();

            match self.current_page.as_str() {
                "Settings" => {
                    ui.group(|ui| {
                        ui.label("Theme Selection");
                        if let Ok(mut data) = self.state.try_lock() {
                            ui.radio_value(&mut data.dark_mode, true, "Dark Mode");
                            ui.radio_value(&mut data.dark_mode, false, "Light Mode");
                        }
                    });
                },
                _ => {
                    if let Ok(data) = self.state.try_lock() {
                        ui.label(format!("Server Status: {}", data.server_status));
                        ui.label(format!("API Hits: {}", data.api_hits));
                        ui.add_space(20.0);
                        ui.label("Latest Logs:");
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            for log in data.logs.iter().rev().take(5) {
                                ui.label(egui::RichText::new(format!("[{}] Access from {}", log.time, log.ip)).monospace().size(10.0));
                            }
                        });
                    }
                }
            }
        });

        ctx.request_repaint();
    }
}

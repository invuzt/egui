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
            current_page: "Home".to_string() 
        }
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut show_panel = true;
        let mut dark_mode = true;
        
        if let Ok(data) = self.state.try_lock() {
            show_panel = data.show_panel;
            dark_mode = data.dark_mode;
        }

        // Paksa tema gelap agar aura terminal terasa
        ctx.set_visuals(egui::Visuals::dark());

        // --- SIDEBAR (Ramping & Rata Kiri) ---
        if show_panel {
            egui::SidePanel::left("sidebar").default_width(110.0).resizable(false).show(ctx, |ui| {
                ui.add_space(45.0);
                ui.vertical_centered_justified(|ui| {
                    if ui.selectable_label(self.current_page == "Home", "🏠 Home").clicked() { self.current_page = "Home".to_string(); }
                    ui.add_space(10.0);
                    if ui.selectable_label(self.current_page == "Settings", "⚙ Settings").clicked() { self.current_page = "Settings".to_string(); }
                });
            });
        }

        // --- MAIN PANEL ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(45.0);
            
            ui.horizontal(|ui| {
                let toggle_icon = if show_panel { "◀" } else { "☰" };
                if ui.button(toggle_icon).clicked() {
                    if let Ok(mut data) = self.state.try_lock() { data.show_panel = !data.show_panel; }
                }
                ui.heading("ODFIZ ENGINE v1.0");
            });
            ui.separator();

            if self.current_page == "Settings" {
                ui.label("Settings Page Content");
            } else if let Ok(data) = self.state.try_lock() {
                
                // --- SIMULASI TUI BOX 1: STATUS ---
                egui::Frame::group(ui.style())
                    .fill(egui::Color32::from_rgb(10, 10, 10))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::YELLOW))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(egui::RichText::new(" [ ENGINE STATUS ] ").color(egui::Color32::YELLOW).strong());
                        ui.label(egui::RichText::new(format!(" STATUS : {}", data.server_status)).monospace());
                    });

                ui.add_space(10.0);

                // --- SIMULASI TUI BOX 2: STATS ---
                egui::Frame::group(ui.style())
                    .fill(egui::Color32::from_rgb(10, 10, 10))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::CYAN))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(egui::RichText::new(" [ STATS ] ").color(egui::Color32::CYAN).strong());
                        ui.label(egui::RichText::new(format!(" HITS   : {}", data.api_hits)).monospace());
                        ui.label(egui::RichText::new(" UPTIME : 100% ").monospace());
                    });

                ui.add_space(10.0);

                // --- SIMULASI TUI BOX 3: LOGS (SCROLLABLE) ---
                ui.label(egui::RichText::new(" [ ACCESS_LOG ] ").strong());
                egui::Frame::canvas(ui.style())
                    .fill(egui::Color32::BLACK)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            for log in data.logs.iter().rev().take(15) {
                                ui.label(egui::RichText::new(format!(" > {} | {}", log.time, log.ip))
                                    .monospace()
                                    .color(egui::Color32::GREEN)
                                    .size(11.0));
                            }
                        });
                    });
            }
        });
        ctx.request_repaint();
    }
}

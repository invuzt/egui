use eframe::egui;
use crate::state::SharedState;

pub struct OdfizApp {
    pub state: SharedState,
    pub current_page: String,
}

// Helper untuk warna TUI agar tidak ngetik RGB terus
const TUI_CYAN: egui::Color32 = egui::Color32::from_rgb(0, 255, 255);
const TUI_YELLOW: egui::Color32 = egui::Color32::from_rgb(255, 255, 0);
const TUI_BLACK: egui::Color32 = egui::Color32::from_rgb(10, 10, 10);

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
        
        if let Ok(data) = self.state.try_lock() {
            show_panel = data.show_panel;
        }

        ctx.set_visuals(egui::Visuals::dark());

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
                ui.label("Settings Page");
            } else if let Ok(data) = self.state.try_lock() {
                
                // BOX 1: STATUS
                egui::Frame::group(ui.style())
                    .fill(TUI_BLACK)
                    .stroke(egui::Stroke::new(1.0, TUI_YELLOW))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(egui::RichText::new(" [ ENGINE STATUS ] ").color(TUI_YELLOW).strong());
                        ui.label(egui::RichText::new(format!(" STATUS : {}", data.server_status)).monospace());
                    });

                ui.add_space(10.0);

                // BOX 2: STATS (Ganti egui::Color32::CYAN jadi TUI_CYAN)
                egui::Frame::group(ui.style())
                    .fill(TUI_BLACK)
                    .stroke(egui::Stroke::new(1.0, TUI_CYAN))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(egui::RichText::new(" [ STATS ] ").color(TUI_CYAN).strong());
                        ui.label(egui::RichText::new(format!(" HITS   : {}", data.api_hits)).monospace());
                        ui.label(egui::RichText::new(" UPTIME : 100% ").monospace());
                    });

                ui.add_space(10.0);

                // BOX 3: LOGS
                ui.label(egui::RichText::new(" [ ACCESS_LOG ] ").strong());
                egui::Frame::canvas(ui.style())
                    .fill(egui::Color32::BLACK)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
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

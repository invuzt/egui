use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use crate::state::SharedState;

pub struct OdfizApp {
    pub state: SharedState,
    pub history: Vec<f64>, // Untuk menyimpan data grafik
}

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        Self { 
            state,
            history: Vec::new(),
        }
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
                    // --- ANIMASI STATUS ---
                    // Warna berdenyut lembut menggunakan fungsi sin dari waktu
                    let pulse = (ctx.input(|i| i.time).sin() + 1.0) / 2.0;
                    let status_color = egui::Color32::from_rgb(
                        (100.0 + 155.0 * pulse) as u8, 
                        255, 
                        (100.0 + 155.0 * pulse) as u8
                    );

                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label("Server:");
                            ui.label(egui::RichText::new(&data.server_status).color(status_color).monospace());
                        });
                        ui.label(format!("API Hits: {}", data.api_hits));
                        
                        // Update data history untuk grafik
                        if self.history.len() > 50 { self.history.remove(0); }
                        self.history.push(data.api_hits as f64);
                    });
                }

                ui.add_space(10.0);
                ui.label(egui::RichText::new("LIVE ENGINE MONITOR").weak().size(9.0));

                // --- EGUI PLOT: Grafik Real-time ---
                let points: PlotPoints = self.history.iter().enumerate()
                    .map(|(i, &val)| [i as f64, val])
                    .collect();
                let line = Line::new(points).color(egui::Color32::from_rgb(0, 255, 150)).fill(0.0);

                Plot::new("server_plot")
                    .view_aspect(2.0) // Buat layout agak gepeng (lite)
                    .show_axes([false, true])
                    .allow_drag(false)
                    .allow_zoom(false)
                    .show(ui, |plot_ui| plot_ui.line(line));

                ui.add_space(10.0);
                if ui.button("RESET COUNTER").clicked() {
                    if let Ok(mut data) = self.state.try_lock() {
                        data.api_hits = 0;
                        self.history.clear();
                    }
                }
            });
        });
        
        ctx.request_repaint(); // Paksa refresh terus menerus untuk animasi halus
    }
}

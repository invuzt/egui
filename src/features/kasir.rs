use eframe::egui;
use eframe::egui::{Color32, RichText};
use egui_plot::{Line, Plot, PlotPoints};
use crate::theme;

pub struct KasirModule {
    items: Vec<(String, u64)>,
    cart: Vec<(String, u64)>,
    data_points: Vec<f64>,
    counter: f64,
}

impl KasirModule {
    pub fn new() -> Self {
        Self {
            items: vec![
                ("PROD-ALPHA".into(), 15000),
                ("PROD-BRAVO".into(), 25000),
                ("PROD-CHARLIE".into(), 50000),
            ],
            cart: Vec::new(),
            data_points: vec![0.0; 60],
            counter: 0.0,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // MEMPERLAMBAT ANIMASI: Update hanya sedikit berdasarkan waktu
        self.counter += 0.05; // Nilai lebih kecil = lebih lambat
        self.data_points.remove(0);
        self.data_points.push((self.counter).sin() * 0.5 + (self.counter * 0.3).cos() * 0.3);
        
        // Meminta repaint sekitar 30fps saja untuk hemat baterai
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(33));

        ui.vertical(|ui| {
            // PLOT DI ATAS MENU
            ui.horizontal(|ui| {
                theme::draw_hud_icon(ui, 18.0, Color32::WHITE);
                ui.label(RichText::new("LIVE FEED DATA STREAM").size(10.0).strong());
            });
            ui.add_space(5.0);

            let plot_data: PlotPoints = self.data_points.iter().enumerate()
                .map(|(i, &v)| [i as f64, v])
                .collect();
            
            Plot::new("kasir_plot")
                .height(100.0)
                .allow_zoom(false)
                .show_axes([false, false])
                .show_grid([true, false])
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new(plot_data).color(Color32::WHITE).width(1.5));
                });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(15.0);

            // MENU SELECTION
            ui.label(RichText::new("TERMINAL SELECTION").size(14.0).strong());
            ui.add_space(10.0);
            for (name, price) in &self.items {
                if ui.add(egui::Button::new(format!(">> {} [RP {}]", name, price))
                    .min_size(egui::vec2(ui.available_width(), 48.0))
                    .rounding(15.0)
                    .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 25))).clicked() {
                    self.cart.push((name.clone(), *price));
                }
                ui.add_space(8.0);
            }

            ui.add_space(15.0);
            let total: u64 = self.cart.iter().map(|(_, p)| p).sum();
            ui.horizontal(|ui| {
                ui.label("TOTAL_SUM:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("Rp {}", total)).size(24.0).strong());
                });
            });
        });
    }
}

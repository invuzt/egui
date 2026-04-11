use eframe::egui;
use eframe::egui::{Color32, RichText};
use egui_plot::{Line, Plot, PlotPoints};
use crate::theme;

pub struct KasirModule {
    items: Vec<(String, u64)>,
    cart: Vec<(String, u64)>,
    data_points: Vec<f64>, // Untuk data plot
}

impl KasirModule {
    pub fn new() -> Self {
        Self {
            items: vec![
                ("Produk A".into(), 15000),
                ("Produk B".into(), 25000),
                ("Produk C".into(), 50000),
            ],
            cart: Vec::new(),
            data_points: (0..50).map(|i| (i as f64).sin()).collect(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Animasi dummy: geser data sedikit demi sedikit
        let last = self.data_points.remove(0);
        self.data_points.push((self.data_points.len() as f64 * 0.1).sin() + last * 0.1);
        ui.ctx().request_repaint(); // Meminta update frame berikutnya agar animasi jalan

        ui.vertical(|ui| {
            // --- PLOT AREA (DASHBOARD STYLE) ---
            ui.label(RichText::new("DATA STREAM v1.0").size(12.0).color(Color32::from_rgb(200, 200, 200)));
            
            let plot_data: PlotPoints = self.data_points.iter().enumerate()
                .map(|(i, &v)| [i as f64, v])
                .collect();
            
            let line = Line::new(plot_data)
                .color(Color32::WHITE)
                .width(2.0);

            Plot::new("kasir_plot")
                .height(120.0)
                .allow_zoom(false)
                .allow_drag(false)
                .show_axes([false, false])
                .show_grid([true, false])
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(10.0);

            // --- POS LOGIC ---
            ui.label(RichText::new("TERMINAL INPUT").strong().color(Color32::WHITE));
            for (name, price) in &self.items {
                if ui.add(egui::Button::new(format!("+ {} (Rp {})", name, price))
                    .min_size(egui::vec2(ui.available_width(), 45.0))
                    .rounding(15.0)
                    .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 30))).clicked() {
                    self.cart.push((name.clone(), *price));
                }
                ui.add_space(8.0);
            }

            ui.add_space(15.0);
            let total: u64 = self.cart.iter().map(|(_, p)| p).sum();
            ui.horizontal(|ui| {
                ui.label(RichText::new("TOTAL:").strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("Rp {}", total)).size(22.0).strong().color(Color32::WHITE));
                });
            });
            
            if ui.button(RichText::new("RESET TRANSACTION").color(Color32::LIGHT_RED)).clicked() {
                self.cart.clear();
            }
        });
    }
}

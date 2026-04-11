use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

pub struct KasirModule {
    items: Vec<(String, u64)>,
    cart: Vec<(String, u64)>,
    data_points: Vec<f64>,
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
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Data stream animasi lambat
        self.data_points.remove(0);
        self.data_points.push((ui.ctx().input(|i| i.time) * 1.5).sin() * 0.5);
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(50));

        ui.vertical(|ui| {
            ui.label(egui::RichText::new("SYSTEM MONITOR").strong());
            
            // Plot Area
            let plot_data: PlotPoints = self.data_points.iter().enumerate()
                .map(|(i, &v)| [i as f64, v]).collect();
            
            Plot::new("kasir_plot")
                .height(80.0)
                .show_axes([false, false])
                .show_grid([true, false])
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new(plot_data).color(egui::Color32::WHITE).width(1.5));
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Menu Buttons
            for (name, price) in &self.items {
                // Button otomatis ikut tema (rounding & padding)
                if ui.add_sized([ui.available_width(), 45.0], egui::Button::new(format!(">> {} [Rp {}]", name, price))).clicked() {
                    self.cart.push((name.clone(), *price));
                }
            }

            ui.add_space(10.0);
            let total: u64 = self.cart.iter().map(|(_, p)| p).sum();
            ui.horizontal(|ui| {
                ui.label("TOTAL:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("Rp {}", total)).size(20.0).strong());
                });
            });
        });
    }
}

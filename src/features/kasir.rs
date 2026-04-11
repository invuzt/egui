use eframe::egui;
use eframe::egui::{Color32, RichText};
use crate::theme;

pub struct KasirModule {
    items: Vec<(String, u64)>,
    cart: Vec<(String, u64)>,
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
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(RichText::new("DAFTAR PRODUK").strong().color(theme::COLOR_ACCENT));
            ui.add_space(10.0);
            for (name, price) in &self.items {
                if ui.add(egui::Button::new(format!("+ {} (Rp {})", name, price))
                    .min_size(egui::vec2(ui.available_width(), 40.0)).rounding(10.0)).clicked() {
                    self.cart.push((name.clone(), *price));
                }
                ui.add_space(5.0);
            }
            ui.add_space(15.0);
            ui.separator();
            ui.label(RichText::new("TOTAL BELANJA:").strong());
            let total: u64 = self.cart.iter().map(|(_, p)| p).sum();
            ui.label(RichText::new(format!("Rp {}", total)).size(24.0).strong().color(Color32::GREEN));
            if ui.button("CLEAR").clicked() { self.cart.clear(); }
        });
    }
}

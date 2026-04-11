use eframe::egui;

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
            for (name, price) in &self.items {
                if ui.button(format!("+ {} (Rp {})", name, price)).clicked() {
                    self.cart.push((name.clone(), *price));
                }
            }

            ui.add_space(10.0);
            let total: u64 = self.cart.iter().map(|(_, p)| p).sum();
            ui.label(egui::RichText::new(format!("Total: Rp {}", total)).size(20.0).strong());
            
            if ui.button("Clear Cart").clicked() {
                self.cart.clear();
            }
        });
    }
}

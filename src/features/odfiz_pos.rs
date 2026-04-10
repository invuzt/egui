use eframe::egui;
use eframe::egui::{Color32, RichText, vec2};
use super::OdfizModule;

pub struct OdfizPOS {
    selected_category: String,
    selected_payment: String,
}

impl OdfizPOS {
    pub fn new() -> Self {
        Self {
            selected_category: "None".to_string(),
            selected_payment: "Cash".to_string(),
        }
    }
}

impl OdfizModule for OdfizPOS {
    fn name(&self) -> &str {
        "Odfiz POS"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // --- PILIHAN KATEGORI ---
            ui.label(RichText::new("PILIH KATEGORI:").color(Color32::LIGHT_GRAY).size(14.0));
            ui.add_space(10.0);
            
            ui.horizontal_wrapped(|ui| {
                let categories = vec!["Retail", "Service", "Food", "Tech"];
                for cat in categories {
                    let is_selected = self.selected_category == cat;
                    let btn = egui::Button::new(cat)
                        .min_size(vec2(80.0, 40.0))
                        .fill(if is_selected { Color32::from_rgb(244, 63, 94) } else { Color32::from_rgb(45, 45, 45) })
                        .rounding(8.0);
                    
                    if ui.add(btn).clicked() {
                        self.selected_category = cat.to_string();
                    }
                }
            });

            ui.add_space(25.0);
            ui.separator();
            ui.add_space(25.0);

            // --- PILIHAN PEMBAYARAN ---
            ui.label(RichText::new("METODE PEMBAYARAN:").color(Color32::LIGHT_GRAY).size(14.0));
            ui.add_space(10.0);

            let payments = vec!["Cash", "Transfer", "QRIS"];
            for pay in payments {
                let is_selected = self.selected_payment == pay;
                if ui.selectable_label(is_selected, format!(" • {}", pay)).clicked() {
                    self.selected_payment = pay.to_string();
                }
            }

            ui.add_space(30.0);

            // --- TOMBOL AKSI UTAMA ---
            if ui.add(egui::Button::new(RichText::new("PROSES TRANSAKSI").strong())
                .fill(Color32::from_rgb(34, 197, 94)) // Hijau
                .min_size(vec2(ui.available_width(), 50.0))
                .rounding(12.0)).clicked() {
                // Logika proses di sini
            }
        });
    }
}

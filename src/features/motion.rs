use eframe::egui;
use eframe::egui::{Color32, Rect, Shape, Stroke, vec2, RichText};
use super::OdfizModule;
use crate::theme;
use std::time::Instant;

pub struct MotionGraphic {
    start_time: Instant,
}

impl MotionGraphic {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    // Fungsi Easing Sederhana (EaseOutExpo)
    fn ease_out_expo(t: f32) -> f32 {
        if t == 1.0 { 1.0 } else { 1.0 - (2.0f32.powf(-10.0 * t)) }
    }
}

impl OdfizModule for MotionGraphic {
    fn name(&self) -> &str { "Motion Graphic" }

    fn ui(&mut self, ui: &mut egui::Ui) {
        // Minta egui repainting terus agar animasi jalan
        ui.ctx().request_repaint();

        let time = self.start_time.elapsed().as_secs_f32();
        
        // Reset waktu jika sudah lewat 3 detik (loop)
        if time > 3.0 {
            self.start_time = Instant::now();
        }

        ui.vertical_centered(|ui| {
            theme::odfiz_card(ui, |ui| {
                ui.label(RichText::new("OFFSET & DELAY").strong().color(theme::COLOR_ACCENT));
                ui.add_space(20.0);

                // Kotak area animasi
                let desired_size = vec2(ui.available_width() * 0.9, 150.0);
                let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
                
                // Background Area
                ui.painter().rect_filled(rect, 10.0, Color32::from_rgb(10, 10, 10));

                // Gambar Elemen Kiri & Kanan dengan Offset & Delay
                let progress = self.ease_out_expo(time.min(1.0)); // Progress 0.0 - 1.0
                let delay_progress = self.ease_out_expo((time - 0.2).max(0.0).min(1.0)); // Progress tertunda

                // Elemen Kiri (Pink-Ungu)
                let x_left = rect.left() + 20.0 + (progress * (rect.width() / 2.0 - 50.0));
                let rect_left = Rect::from_center_size(
                    vec2(x_left, rect.center().y()),
                    vec2(60.0, 40.0)
                );
                ui.painter().rect_filled(rect_left, 10.0, theme::COLOR_ACCENT);

                // Elemen Kanan (Ungu) - Tertunda
                let x_right = rect.right() - 20.0 - (delay_progress * (rect.width() / 2.0 - 50.0));
                let rect_right = Rect::from_center_size(
                    vec2(x_right, rect.center().y()),
                    vec2(60.0, 40.0)
                );
                ui.painter().rect_filled(rect_right, 10.0, Color32::from_rgb(107, 33, 168));

            });

            ui.add_space(20.0);
            
            if ui.button("Reset Animasi").clicked() {
                self.start_time = Instant::now();
            }
        });
    }
}

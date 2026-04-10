use eframe::egui;
use eframe::egui::{Color32, RichText, vec2};
use super::OdfizModule;

pub struct OdfizCalc {
    display: String,
}

impl OdfizCalc {
    pub fn new() -> Self {
        Self { display: "0".to_string() }
    }

    fn add_digit(&mut self, digit: &str) {
        if self.display == "0" { self.display = digit.to_string(); }
        else { self.display.push_str(digit); }
    }
}

impl OdfizModule for OdfizCalc {
    fn name(&self) -> &str { "Calculator" }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            // Screen Calculator
            egui::Frame::none().fill(Color32::from_rgb(10, 10, 10)).inner_margin(20.0).rounding(10.0).show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(&self.display).size(40.0).strong().color(Color32::WHITE));
                });
            });

            ui.add_space(20.0);

            // Grid Tombol
            let buttons = [
                ["7", "8", "9", "/"],
                ["4", "5", "6", "*"],
                ["1", "2", "3", "-"],
                ["C", "0", "=", "+"],
            ];

            for row in buttons {
                ui.horizontal(|ui| {
                    for txt in row {
                        let btn = egui::Button::new(RichText::new(txt).size(20.0))
                            .min_size(vec2(60.0, 60.0))
                            .fill(if "/*-+=C".contains(txt) { Color32::from_rgb(244, 63, 94) } else { Color32::from_rgb(45, 45, 45) })
                            .rounding(30.0); // Bulat sempurna
                        
                        if ui.add(btn).clicked() {
                            match txt {
                                "C" => self.display = "0".to_string(),
                                "=" => { /* Logika eval sederhana bisa ditambah */ },
                                _ => self.add_digit(txt),
                            }
                        }
                    }
                });
                ui.add_space(10.0);
            }
        });
    }
}

#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use eframe::egui::{Color32, Visuals, TextStyle, FontId, FontFamily, Frame, RichText, vec2};
use features::get_all_modules;

struct OdfizShell {
    modules: Vec<(bool, Box<dyn features::OdfizModule>)>,
    active_feature: Option<usize>,
}

const COLOR_BG: Color32 = Color32::from_rgb(15, 15, 15);
const COLOR_CARD_OUTER: Color32 = Color32::from_rgb(25, 25, 25);
const COLOR_CARD_INNER: Color32 = Color32::from_rgb(35, 35, 35);
const COLOR_ACCENT: Color32 = Color32::from_rgb(244, 63, 94);
const COLOR_TEXT_DIM: Color32 = Color32::from_rgb(120, 120, 120);

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Shell",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.visuals = Visuals::dark();
            style.visuals.panel_fill = COLOR_BG;
            style.text_styles.insert(TextStyle::Heading, FontId::new(26.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Body, FontId::new(18.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Button, FontId::new(18.0, FontFamily::Proportional));
            cc.egui_ctx.set_style(style);

            Box::new(OdfizShell { 
                modules: get_all_modules(),
                active_feature: None,
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.active_feature.is_none() {
            // --- DASHBOARD MODE ---
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(50.0);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("ODFIZ CORE SYSTEM").strong().size(20.0).color(COLOR_ACCENT));
                    ui.add_space(20.0);
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        for (i, (_, module)) in self.modules.iter_mut().enumerate() {
                            Frame::none()
                                .fill(COLOR_CARD_OUTER)
                                .inner_margin(20.0)
                                .rounding(18.0)
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width() * 0.92);
                                    
                                    // Info Inner Card
                                    Frame::none().fill(COLOR_CARD_INNER).inner_margin(12.0).rounding(10.0)
                                        .stroke(egui::Stroke::new(1.0, COLOR_ACCENT)).show(ui, |ui| {
                                            ui.set_min_width(ui.available_width());
                                            ui.label(RichText::new("MODULE NAME").color(COLOR_TEXT_DIM).size(10.0));
                                            ui.label(RichText::new(module.name()).strong().size(18.0));
                                        });

                                    ui.add_space(15.0);
                                    
                                    let btn = egui::Button::new(RichText::new("OPEN MODULE").color(Color32::WHITE).strong())
                                        .fill(COLOR_ACCENT).min_size(vec2(ui.available_width(), 45.0)).rounding(10.0);
                                    
                                    if ui.add(btn).clicked() {
                                        self.active_feature = Some(i);
                                    }
                                });
                            ui.add_space(20.0);
                        }
                    });
                });
            });
        } else {
            // --- FEATURE PAGE MODE ---
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(50.0);
                
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("← Back").size(18.0).color(COLOR_ACCENT)).clicked() {
                        self.active_feature = None;
                    }
                    if let Some(i) = self.active_feature {
                        ui.label(RichText::new(self.modules[i].1.name()).heading().strong());
                    }
                });
                
                ui.add_space(20.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Bungkus isi modul dengan style card juga
                    Frame::none()
                        .fill(COLOR_CARD_OUTER)
                        .inner_margin(20.0)
                        .rounding(15.0)
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            if let Some(i) = self.active_feature {
                                self.modules[i].1.ui(ui);
                            }
                        });
                });
            });
        }
    }
}

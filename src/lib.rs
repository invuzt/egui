#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use eframe::egui::{Color32, Visuals, TextStyle, FontId, FontFamily, Frame, RichText, vec2};
use features::get_all_modules;

struct OdfizShell {
    modules: Vec<(bool, Box<dyn features::OdfizModule>)>,
    active_feature: Option<usize>,
}

// Konstanta Warna ala Mockup
const COLOR_BG: Color32 = Color32::from_rgb(15, 15, 15);
const COLOR_CARD_OUTER: Color32 = Color32::from_rgb(25, 25, 25);
const COLOR_CARD_INNER: Color32 = Color32::from_rgb(35, 35, 35);
const COLOR_ACCENT: Color32 = Color32::from_rgb(244, 63, 94); // Pink-Red
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
            
            // Setup Font Besar
            style.text_styles.insert(TextStyle::Heading, FontId::new(24.0, FontFamily::Proportional));
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
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(50.0); // Status bar area
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        for (i, (_, module)) in self.modules.iter_mut().enumerate() {
                            
                            // --- OUTER CARD ---
                            Frame::none()
                                .fill(COLOR_CARD_OUTER)
                                .inner_margin(25.0)
                                .rounding(20.0)
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width() * 0.9);
                                    
                                    // Header Text (Pink)
                                    ui.label(RichText::new("ODFIZ MODULE").color(COLOR_ACCENT).strong().size(16.0));
                                    ui.add_space(15.0);

                                    // --- INNER CARD (Info Box) ---
                                    Frame::none()
                                        .fill(COLOR_CARD_INNER)
                                        .inner_margin(15.0)
                                        .rounding(12.0)
                                        .stroke(egui::Stroke::new(1.0, COLOR_ACCENT)) // Border tipis pink di kiri/atas ala mockup
                                        .show(ui, |ui| {
                                            ui.set_min_width(ui.available_width());
                                            ui.vertical(|ui| {
                                                ui.label(RichText::new("BRAND").color(COLOR_TEXT_DIM).size(12.0));
                                                ui.label(RichText::new("Odfiz Tech").strong());
                                                ui.add_space(8.0);

                                                ui.label(RichText::new("MODULE").color(COLOR_TEXT_DIM).size(12.0));
                                                ui.label(RichText::new(module.name()).strong());
                                                ui.add_space(8.0);

                                                ui.label(RichText::new("STATUS").color(COLOR_TEXT_DIM).size(12.0));
                                                ui.label(RichText::new("Online").color(Color32::from_rgb(34, 197, 94)).strong());
                                            });
                                        });

                                    ui.add_space(20.0);

                                    // --- BUTTON REFRESH/MASUK ---
                                    let btn = egui::Button::new(RichText::new("OPEN MODULE").color(Color32::WHITE).strong())
                                        .fill(COLOR_ACCENT)
                                        .min_size(vec2(ui.available_width(), 50.0))
                                        .rounding(12.0);

                                    if ui.add(btn).clicked() {
                                        self.active_feature = Some(i);
                                    }
                                });
                            ui.add_space(30.0);
                        }
                    });
                });
            });
        } else {
            // Tampilan Modul yang Aktif
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(50.0);
                if ui.button("← Back to Dashboard").clicked() {
                    self.active_feature = None;
                }
                ui.separator();
                if let Some(i) = self.active_feature {
                    self.modules[i].1.ui(ui);
                }
            });
        }
    }
}

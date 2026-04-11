#![cfg(target_os = "android")]
mod features;
mod theme;

use eframe::egui;
use eframe::egui::RichText;
use features::get_all_modules;

struct OdfizShell {
    module_states: Vec<bool>,
    modules: Vec<Box<dyn features::OdfizModule>>,
}

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
            style.visuals = egui::Visuals::dark();
            style.visuals.panel_fill = theme::COLOR_BG;
            
            // Perbesar Font Default Global
            style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(20.0, egui::FontFamily::Proportional));
            style.text_styles.insert(egui::TextStyle::Button, egui::FontId::new(20.0, egui::FontFamily::Proportional));
            
            cc.egui_ctx.set_style(style);

            let all = get_all_modules();
            let count = all.len();
            
            Box::new(OdfizShell { 
                module_states: vec![false; count],
                modules: all.into_iter().map(|(_, m)| m).collect(),
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(60.0); // Lebih turun dikit dari status bar
            
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("ODFIZ CORE SYSTEM")
                    .strong()
                    .size(20.0) // Judul lebih besar
                    .color(theme::COLOR_ACCENT)
                    .extra_letter_spacing(3.0));
                ui.add_space(30.0);
            });

            egui::ScrollArea::vertical()
                .id_source("main_scroll")
                .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    for (i, module) in self.modules.iter_mut().enumerate() {
                        let is_open = self.module_states[i];

                        theme::odfiz_card(ui, |ui: &mut egui::Ui| {
                            ui.set_width(ui.available_width() * 0.94);
                            
                            ui.horizontal(|ui: &mut egui::Ui| {
                                ui.label(RichText::new(module.name().to_uppercase())
                                    .strong()
                                    .size(22.0) // Nama modul di kartu lebih besar
                                    .color(if is_open { theme::COLOR_ACCENT } else { egui::Color32::WHITE }));
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui: &mut egui::Ui| {
                                    let icon = if is_open { "🔼" } else { "🔽" };
                                    // Tombol panah lebih besar
                                    if ui.button(RichText::new(icon).size(20.0)).clicked() {
                                        self.module_states[i] = !is_open;
                                    }
                                });
                            });

                            if is_open {
                                ui.add_space(20.0);
                                ui.separator();
                                ui.add_space(20.0);
                                
                                ui.vertical(|ui: &mut egui::Ui| {
                                    // Konten modul (Server Lite) akan ikut besar
                                    module.ui(ui);
                                });
                                
                                ui.add_space(20.0);
                                if ui.button(RichText::new("CLOSE MENU").size(14.0).color(egui::Color32::GRAY)).clicked() {
                                    self.module_states[i] = false;
                                }
                            }
                        });
                        ui.add_space(20.0); // Jarak antar kartu lebih lega
                    }
                });
            });
        });
    }
}

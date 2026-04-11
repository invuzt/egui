#![cfg(target_os = "android")]
mod features;
mod theme;

use eframe::egui;
use eframe::egui::{RichText, Frame};
use features::get_all_modules;

struct OdfizShell {
    // Kita simpan status buka/tutup tiap modul di sini
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
            cc.egui_ctx.set_style(style);

            let all = get_all_modules();
            let count = all.len();
            
            Box::new(OdfizShell { 
                module_states: vec![false; count], // Semua tertutup di awal
                modules: all.into_iter().map(|(_, m)| m).collect(),
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(50.0);
            
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("ODFIZ CORE SYSTEM")
                    .strong()
                    .size(16.0)
                    .color(theme::COLOR_ACCENT)
                    .extra_letter_spacing(2.0));
                ui.add_space(20.0);
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    // Iterasi semua modul
                    for (i, module) in self.modules.iter_mut().enumerate() {
                        let is_open = self.module_states[i];

                        theme::odfiz_card(ui, |ui| {
                            ui.set_width(ui.available_width() * 0.95);
                            
                            // Baris Judul & Tombol Toggle
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(module.name().to_uppercase())
                                    .strong()
                                    .color(if is_open { theme::COLOR_ACCENT } else { egui::Color32::WHITE }));
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let icon = if is_open { "🔼" } else { "🔽" };
                                    if ui.button(RichText::new(icon).size(14.0)).clicked() {
                                        self.module_states[i] = !is_open; // Toggle buka/tutup
                                    }
                                });
                            });

                            // JIKA TERBUKA: Tampilkan konten di bawahnya
                            if is_open {
                                ui.add_space(15.0);
                                ui.separator();
                                ui.add_space(15.0);
                                
                                // Bungkus konten modul biar rapi
                                ui.vertical(|ui| {
                                    module.ui(ui);
                                });
                                
                                ui.add_space(10.0);
                                if ui.button(RichText::new("TUTUP MENU").size(12.0).color(theme::COLOR_TEXT_DIM)).clicked() {
                                    self.module_states[i] = false;
                                }
                            }
                        });
                        ui.add_space(15.0);
                    }
                });
            });
        });
    }
}

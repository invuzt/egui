#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use eframe::egui::{FontId, FontFamily, TextStyle};
use features::get_all_modules;

struct OdfizShell {
    modules: Vec<(bool, Box<dyn features::OdfizModule>)>,
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
            // --- SETTING UKURAN GLOBAL ---
            let mut style = (*cc.egui_ctx.style()).clone();
            
            // Perbesar semua jenis tulisan
            style.text_styles.insert(TextStyle::Heading, FontId::new(30.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Body, FontId::new(22.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Button, FontId::new(22.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Monospace, FontId::new(20.0, FontFamily::Monospace));
            
            // Perbesar spasi antar tombol dan padding tombol
            style.spacing.item_spacing = egui::vec2(10.0, 25.0);
            style.spacing.button_padding = egui::vec2(20.0, 15.0);
            style.spacing.interact_size = egui::vec2(40.0, 40.0); // Area klik minimal jadi lebih luas

            cc.egui_ctx.set_style(style);

            Box::new(OdfizShell { 
                modules: get_all_modules(),
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Gunakan scroll area agar kalau modul banyak tidak terpotong
            egui::ScrollArea::vertical()
                .id_source("main_scroll")
                .show(ui, |ui| {
                    ui.add_space(50.0); // Jarak aman status bar atas
                    
                    // MEMBUAT SEMUA RATA TENGAH
                    ui.vertical_centered(|ui| {
                        ui.heading("ODFIZ APP");
                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(20.0);

                        for (enabled, module) in self.modules.iter_mut() {
                            // Checkbox dibuat besar dan di tengah
                            ui.checkbox(enabled, module.name());
                            
                            if *enabled {
                                ui.add_space(10.0);
                                // Isi modul juga dipaksa rata tengah
                                ui.vertical_centered(|ui| {
                                    module.ui(ui);
                                });
                                ui.add_space(20.0);
                                ui.separator();
                                ui.add_space(20.0);
                            }
                        }
                    });
                });
        });
    }
}

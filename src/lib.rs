#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use eframe::egui::{Visuals, TextStyle, FontId, FontFamily};
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
            // --- KUSTOMISASI TAMPILAN MODERN ---
            let mut style = (*cc.egui_ctx.style()).clone();
            
            // 1. Pakai Tema Terang (Light Mode) agar seperti aplikasi standar
            style.visuals = Visuals::light();
            
            // 2. Perbesar Font agar mudah dibaca di layar HP
            style.text_styles.insert(TextStyle::Heading, FontId::new(24.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Body, FontId::new(18.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Button, FontId::new(18.0, FontFamily::Proportional));
            
            // 3. Modern Rounded Shapes (Tombol & Frame jadi membulat)
            style.visuals.widgets.noninteractive.rounding = 12.0.into();
            style.visuals.widgets.inactive.rounding = 12.0.into();
            style.visuals.widgets.hovered.rounding = 12.0.into();
            style.visuals.widgets.active.rounding = 12.0.into();
            
            // 4. Atur Spasi antar elemen (Padding)
            style.spacing.item_spacing = egui::vec2(12.0, 15.0);
            style.spacing.button_padding = egui::vec2(20.0, 12.0);

            cc.egui_ctx.set_style(style);

            Box::new(OdfizShell { 
                modules: get_all_modules(),
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Area aman Status Bar
        egui::TopBottomPanel::top("top_bar")
            .frame(egui::Frame::none().fill(ctx.style().visuals.panel_fill))
            .show(ctx, |ui| {
                ui.add_space(45.0); 
                ui.vertical_centered(|ui| {
                    ui.heading("ODFIZ SYSTEM");
                });
                ui.add_space(10.0);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_source("main_scroll")
                .show(ui, |ui| {
                    ui.add_space(10.0);
                    
                    // Dashboard Switcher (Horizontal Chips)
                    ui.label("Pilih Layanan:");
                    ui.horizontal_wrapped(|ui| {
                        for (enabled, module) in self.modules.iter_mut() {
                            ui.selectable_label(*enabled, module.name())
                                .clicked()
                                .then(|| *enabled = !*enabled);
                        }
                    });
                    
                    ui.add_space(10.0);
                    ui.separator();
                    
                    // Tampilkan Modul yang aktif dengan desain Card
                    for (enabled, module) in self.modules.iter_mut() {
                        if *enabled {
                            ui.add_space(10.0);
                            egui::Frame::group(ui.style())
                                .fill(ctx.style().visuals.widgets.noninteractive.bg_fill)
                                .inner_margin(15.0)
                                .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    module.ui(ui);
                                });
                        }
                    }
                });
        });
    }
}

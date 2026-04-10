#![cfg(target_os = "android")]
mod features;

use eframe::egui;
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
        Box::new(|_cc| {
            Box::new(OdfizShell { 
                modules: get_all_modules(),
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Space buat status bar biar nggak ketutup
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(40.0);
            ui.heading("Odfiz App");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("Daftar Modul:");
                for (enabled, module) in self.modules.iter_mut() {
                    ui.checkbox(enabled, module.name());
                    if *enabled {
                        ui.indent("module", |ui| {
                            module.ui(ui);
                        });
                        ui.separator();
                    }
                }
            });
        });
    }
}

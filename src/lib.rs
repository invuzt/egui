#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use features::{OdfizModule, counter_feature::CounterFeature, crud_feature::CrudFeature};

struct OdfizShell {
    // Daftar kapling: (Status Aktif, Objek Fitur)
    modules: Vec<(bool, Box<dyn OdfizModule>)>,
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
            // --- DAFTARKAN MODUL BARU DI SINI ---
            let modules: Vec<(bool, Box<dyn OdfizModule>)> = vec![
                (false, Box::new(CounterFeature::new())),
                (false, Box::new(CrudFeature::new())),
            ];
            Box::new(OdfizShell { modules })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ MULTI-TOOL SHELL");
                ui.add_space(10.0);

                // Checkbox otomatis untuk semua modul yang terdaftar
                ui.horizontal_wrapped(|ui| {
                    for (enabled, module) in self.modules.iter_mut() {
                        ui.checkbox(enabled, module.name());
                    }
                });
                
                ui.add_space(10.0);
                ui.separator();
                
                // Area Gambar Fitur
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (enabled, module) in self.modules.iter_mut() {
                        if *enabled {
                            ui.add_space(10.0);
                            module.ui(ui);
                        }
                    }
                });
            });
        });
    }
}

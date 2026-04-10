#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use features::counter_feature::CounterFeature;

struct OdfizShell {
    // Saklar fitur
    is_counter_enabled: bool,
    // Data fitur
    counter_module: CounterFeature,
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
                is_counter_enabled: false,
                counter_module: CounterFeature::new(),
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ MULTI-TOOL SHELL");
                ui.add_space(10.0);

                // Pengaturan Cangkang
                ui.checkbox(&mut self.is_counter_enabled, "Aktifkan Modul Counter");
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(20.0);

                // Logika Injeksi/Loading Fitur
                if self.is_counter_enabled {
                    self.counter_module.ui(ui);
                } else {
                    ui.label("Semua modul non-aktif.");
                }

                ui.add_space(20.0);
                if ui.button("Reset Shell").clicked() {
                    self.is_counter_enabled = false;
                }
            });
        });
    }
}

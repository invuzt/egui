#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use features::counter_feature::CounterFeature;
use features::crud_feature::CrudFeature;

struct OdfizShell {
    is_counter_on: bool,
    is_crud_on: bool,
    counter_mod: CounterFeature,
    crud_mod: CrudFeature,
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
                is_counter_on: false,
                is_crud_on: false,
                counter_mod: CounterFeature::new(),
                crud_mod: CrudFeature::new(),
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ MULTI-TOOL");
                ui.add_space(10.0);

                ui.checkbox(&mut self.is_counter_on, "🔢 Aktifkan Counter");
                ui.checkbox(&mut self.is_crud_on, "📦 Aktifkan Database CRUD");
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(20.0);

                if self.is_counter_on {
                    self.counter_mod.ui(ui);
                    ui.add_space(15.0);
                }

                if self.is_crud_on {
                    self.crud_mod.ui(ui);
                }

                if !self.is_counter_on && !self.is_crud_on {
                    ui.label("Pilih modul untuk memulai.");
                }
            });
        });
    }
}

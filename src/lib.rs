#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use features::get_all_modules;
use egui_keyboard::Keyboard;

struct OdfizShell {
    modules: Vec<(bool, Box<dyn features::OdfizModule>)>,
    keyboard: Keyboard,
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
                keyboard: Keyboard::default(),
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.keyboard.pump_events(ctx);

        // Header agar tidak bentrok status bar
        egui::TopBottomPanel::top("header_spacer")
            .frame(egui::Frame::none())
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(40.0); 
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ MODULAR SHELL");
                ui.add_space(10.0);
                
                ui.horizontal_wrapped(|ui| {
                    for (enabled, module) in self.modules.iter_mut() {
                        ui.checkbox(enabled, module.name());
                    }
                });
                ui.separator();

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

        self.keyboard.show(ctx);
    }
}

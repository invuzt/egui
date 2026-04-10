#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use features::get_all_modules;
use egui_virtual_keyboard::VirtualKeyboard;

struct OdfizShell {
    modules: Vec<(bool, Box<dyn features::OdfizModule>)>,
    keyboard: VirtualKeyboard,
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
            Ok(Box::new(OdfizShell { 
                modules: get_all_modules(),
                keyboard: VirtualKeyboard::default(),
            }))
        }),
    );
}

impl eframe::App for OdfizShell {
    // Hook untuk menyambungkan keyboard virtual
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.keyboard.bump_events(ctx, raw_input);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Safety area status bar
        egui::TopBottomPanel::top("spacer")
            .frame(egui::Frame::none())
            .show(ctx, |ui| ui.add_space(45.0));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ MODULAR");
                
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

        // Tampilkan keyboard jika ada input fokus
        if ctx.wants_keyboard_input() {
            egui::Window::new("KBD")
                .anchor(egui::Align2::CENTER_BOTTOM, [0.0, 0.0])
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .show(ctx, |ui| {
                    self.keyboard.show(ui);
                });
        }
    }
}

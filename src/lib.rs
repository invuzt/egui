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
            Box::new(OdfizShell { 
                modules: get_all_modules(),
                keyboard: VirtualKeyboard::default(),
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    // Kunci utamanya di sini, Mas:
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.keyboard.bump_events(ctx, raw_input);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Padding atas agar tidak kena status bar
        egui::TopBottomPanel::top("spacer").frame(egui::Frame::none()).show(ctx, |ui| {
            ui.add_space(40.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ SUPER APP");
                
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

        // Munculkan keyboard di jendela melayang (Window)
        // Dia otomatis muncul kalau ada widget yang minta input
        if ctx.wants_keyboard_input() {
            egui::Window::new("Papan Ketik")
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

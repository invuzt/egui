#![cfg(target_os = "android")]
mod theme;

use eframe::egui;

struct OdfizHello {
    user_name: String,
    rust_response: String,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Hello",
        options,
        Box::new(|cc| {
            theme::apply_basic_style(&cc.egui_ctx);
            Box::new(OdfizHello { 
                user_name: String::new(),
                rust_response: String::new(),
            })
        }),
    );
}

impl eframe::App for OdfizHello {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.heading("RUST NDK - HELLO WORLD");
                ui.add_space(20.0);

                ui.label("Masukkan nama Anda:");
                let input = ui.text_edit_singleline(&mut self.user_name);
                
                ui.add_space(10.0);

                if ui.button("KIRIM KE RUST").clicked() || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    if self.user_name.trim().is_empty() {
                        self.rust_response = "Rust: Tolong isi namanya dulu ya!".to_string();
                    } else {
                        self.rust_response = format!("Rust bilang: Halo {}, selamat datang di dunia Rust NDK!", self.user_name);
                    }
                }

                if !self.rust_response.is_empty() {
                    ui.add_space(30.0);
                    ui.separator();
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(&self.rust_response).size(18.0).strong());
                }
            });
        });
    }
}

#![cfg(target_os = "android")]
mod features;
mod theme;

use eframe::egui;

struct OdfizShell {
    mm: features::ModuleManager,
    user_name: String,
    greeting: String,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Clean",
        options,
        Box::new(|cc| {
            theme::apply_clean_style(&cc.egui_ctx);
            Box::new(OdfizShell { 
                mm: features::ModuleManager::new(),
                user_name: String::new(),
                greeting: String::new(),
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            
            ui.heading("Odfiz Core - Clean Mode");
            ui.separator();
            ui.add_space(10.0);

            // --- FITUR INPUT NAMA ---
            ui.group(|ui| {
                ui.label("Masukkan Nama Anda:");
                let name_input = ui.text_edit_singleline(&mut self.user_name);
                
                if ui.button("Kirim ke Rust").clicked() || (name_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    if self.user_name.trim().is_empty() {
                        self.greeting = "Rust bilang: Namanya jangan kosong dong, Mas!".to_string();
                    } else {
                        // Di sini Rust memproses input
                        self.greeting = format!("Halo {}, salam dari Rust NDK!", self.user_name);
                    }
                }

                if !self.greeting.is_empty() {
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(&self.greeting).color(egui::Color32::LIGHT_BLUE).strong());
                }
            });

            ui.add_space(20.0);

            // --- MODUL LAIN (Layout Bersih) ---
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.collapsing("🌐 Lite Server Control", |ui| {
                    self.mm.server.ui(ui);
                });

                ui.add_space(10.0);

                ui.collapsing("💰 Kasir System", |ui| {
                    self.mm.kasir.ui(ui);
                });
            });
        });
    }
}

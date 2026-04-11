mod theme;

use miniquad::*;

struct Stage {
    egui_mq: egui_miniquad::EguiBackend,
}

impl Stage {
    fn new() -> Self {
        Self {
            egui_mq: egui_miniquad::EguiBackend::new(),
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        // Clear screen ke warna abu-abu iOS
        let screen = window::screen_size();
        
        self.egui_mq.run(|ctx| {
            theme::apply_ios_style(ctx);

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(100.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("ODFIZ MINIQUAD").size(30.0).strong());
                    ui.add_space(20.0);
                    
                    ui.group(|ui| {
                        ui.label("Status: Pure Rust & Miniquad");
                        ui.label("Size: Extremely Optimized");
                    });

                    ui.add_space(30.0);
                    if ui.button("CEK KONEKSI").clicked() {
                        // Sesuatu di sini nanti
                    }
                });
            });
        });

        // Gambar ke GPU
        self.egui_mq.draw();
    }
}

// Entry point untuk Android
#[no_mangle]
pub extern "C" fn android_main(app: miniquad::native::android::AndroidApp) {
    miniquad::start(conf::Conf::default(), move || {
        Box::new(Stage::new())
    });
}

#![cfg(target_os = "android")]
mod theme;

use eframe::egui;

struct OdfizZero {
    val: f32,
    status: String,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Zero",
        options,
        Box::new(|cc| {
            theme::apply_minimal_style(&cc.egui_ctx);
            Box::new(OdfizZero { 
                val: 50.0,
                status: "Sistem Siap".to_string(),
            })
        }),
    );
}

impl eframe::App for OdfizZero {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TANPA request_repaint() = CPU 0% saat diam
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);

                ui.label("KONTROL POWER");
                
                // Slider sebagai pengganti input angka (Tanpa Keyboard)
                ui.add(egui::Slider::new(&mut self.val, 0.0..=100.0).text("%"));
                
                ui.add_space(20.0);

                ui.horizontal_centered(|ui| {
                    if ui.button(" MIN ").clicked() { 
                        self.val = 0.0;
                        self.status = "Set ke Minimum".to_string();
                    }
                    if ui.button(" MAX ").clicked() { 
                        self.val = 100.0;
                        self.status = "Set ke Maksimum".to_string();
                    }
                });

                ui.add_space(40.0);
                ui.separator();
                ui.add_space(20.0);
                
                ui.label(format!("STATUS: {}", self.status));
                ui.label(format!("OUTPUT RUST: {:.1}", self.val));
            });
        });
    }
}

#![cfg(target_os = "android")]
use eframe::egui;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let mut options = eframe::NativeOptions::default();
    
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    eframe::run_native(
        "Odfiz App",
        options,
        Box::new(|cc| {
            // Memastikan UI cukup besar untuk layar HP
            cc.egui_ctx.set_pixels_per_point(3.0);
            
            // Opsional: Custom font bisa dimuat di sini jika masih kosong
            Box::new(MyApp::default())
        }),
    );
}

struct MyApp {
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { text: "Odfiz Rust".to_owned() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                // Menggunakan heading agar teks lebih besar
                ui.heading("🚀 Odfiz Native Rust");
                
                ui.add_space(20.0);
                ui.text_edit_singleline(&mut self.text);
                
                ui.add_space(20.0);
                if ui.button("Konfirmasi").clicked() {
                    self.text = "Teks Terdeteksi!".to_owned();
                }
                
                ui.add_space(20.0);
                ui.label(format!("Input: {}", self.text));
            });
        });
    }
}

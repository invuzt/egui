#![cfg(target_os = "android")]
use eframe::egui;

#[no_mangle]
fn android_main(app: eframe::winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    
    // Kita berikan closure kosong untuk memastikan renderer siap
    eframe::run_native(
        "Odfiz App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
        app,
    );
}

struct MyApp {
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { name: "Odfiz".to_owned() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Akhirnya Berhasil!");
            ui.label(format!("Halo, {}!", self.name));
            ui.text_edit_singleline(&mut self.name);
        });
    }
}

#![cfg(target_os = "android")]
use eframe::egui;

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let options = eframe::NativeOptions::default();

    // Menggunakan run_android untuk menghindari konflik dengan run_native desktop
    eframe::run_android(
        app,
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ).expect("Gagal menjalankan eframe");
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
            ui.heading("Odfiz Android Berhasil!");
            ui.label(format!("Halo, {}!", self.name));
            ui.text_edit_singleline(&mut self.name);
        });
    }
}

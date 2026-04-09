#![cfg(target_os = "android")]

use eframe::egui;

#[no_mangle]
fn android_main(app: eframe::winit::platform::android::activity::AndroidApp) {
    let options = eframe::NativeOptions::default();

    // Gunakan run_android, bukan run_native
    eframe::run_android(
        app,
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ).unwrap();
}

struct MyApp {
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { name: "Odfiz User".to_owned() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Odfiz Hello World");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label("Nama: ");
                    ui.text_edit_singleline(&mut self.name);
                });
                if ui.button("Proses").clicked() {
                    self.name = "Berhasil!".to_owned();
                }
            });
        });
    }
}

use eframe::egui;
use android_activity::AndroidApp;

#[no_mangle]
fn android_main(app: AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    
    // Trik: Memasukkan AndroidApp ke dalam closure renderer
    // Ini menghindari masalah jumlah argumen pada run_native
    eframe::run_native(
        "Hello Egui",
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
            ui.heading("Odfiz Egui Android");
            ui.text_edit_singleline(&mut self.name);
            if ui.button("Klik").clicked() {
                self.name = "BERHASIL!".to_owned();
            }
        });
    }
}

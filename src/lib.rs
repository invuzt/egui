use eframe::egui;
use android_activity::AndroidApp;

#[no_mangle]
fn android_main(app: AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    
    // Di eframe 0.27, app dimasukkan ke renderer_init_closure atau lewat cara ini:
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
        Self { name: "Odfiz User".to_owned() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Odfiz Hello World");
                ui.add_space(10.0);
                ui.label(format!("Halo, {}!", self.name));
                ui.text_edit_singleline(&mut self.name);
                if ui.button("Klik").clicked() {
                    self.name = "Build Berhasil!".to_owned();
                }
            });
        });
    }
}

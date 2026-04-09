use eframe::{egui, AndroidApp};

#[no_mangle]
fn android_main(app: AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    
    // Cara yang benar untuk passing app di eframe 0.27
    options.android_sdk_config = Some(eframe::AndroidSdkConfig {
        app: Some(app),
    });

    eframe::run_native(
        "Hello Egui",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
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
                ui.horizontal(|ui| {
                    ui.label("Input:");
                    ui.text_edit_singleline(&mut self.name);
                });
                if ui.button("Proses").clicked() {
                    self.name = "Build Berhasil!".to_owned();
                }
            });
        });
    }
}

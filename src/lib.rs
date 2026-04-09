#![cfg(target_os = "android")]
use eframe::egui;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let mut options = eframe::NativeOptions::default();
    
    // Kita buat event loop builder secara manual agar eframe "terpaksa" mengenalinya sebagai Android
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    // Di sini kita gunakan 3 argumen sesuai keinginan compiler Anda
    eframe::run_native(
        "Odfiz App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
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
            ui.heading("Odfiz Build Success!");
            ui.label(format!("Halo, {}!", self.name));
            ui.text_edit_singleline(&mut self.name);
        });
    }
}

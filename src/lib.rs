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
            // SETTING: Memperbesar UI agar tidak kecil di layar HP
            cc.egui_ctx.set_pixels_per_point(3.0); 
            Box::new(MyApp::default())
        }),
    );
}

struct MyApp {
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { name: "Developer".to_owned() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Membuat konten di tengah layar
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.heading("🚀 Odfiz Rust Android");
                ui.add_space(20.0);
                
                ui.group(|ui| {
                    ui.label("Masukkan Nama Anda:");
                    ui.text_edit_singleline(&mut self.name);
                });

                ui.add_space(20.0);
                if ui.button("KLIK SAYA").clicked() {
                    self.name = "Berhasil Push!".to_owned();
                }
                
                ui.add_space(20.0);
                ui.label(format!("Status: {}", self.name));
            });
        });
    }
}

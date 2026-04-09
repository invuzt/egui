#![cfg(target_os = "android")]
use eframe::egui;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let mut options = eframe::NativeOptions::default();
    
    // 1. PAKSA PAKAI OPENGL (Lebih stabil dari WGPU di Android)
    options.renderer = eframe::Renderer::Glow;

    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    // 2. Gunakan catch_unwind agar tidak langsung mati total jika ada panic
    let _ = eframe::run_native(
        "Odfiz App",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(3.0);
            Box::new(MyApp::default())
        }),
    );
}

struct MyApp {
    counter: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.heading("🚀 Odfiz Test Build");
                
                ui.add_space(20.0);
                if ui.button("TAMBAH").clicked() {
                    self.counter += 1;
                }
                
                ui.label(format!("Angka: {}", self.counter));
            });
        });
    }
}

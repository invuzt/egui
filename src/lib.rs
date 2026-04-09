#![cfg(target_os = "android")]
use eframe::egui;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Graphic Test",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    rotation: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { rotation: 0.0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Kita buat animasi rotasi tanpa teks
            self.rotation += 0.02;
            
            let center = ui.max_rect().center();
            let painter = ui.painter();
            
            // Gambar Kotak berputar di tengah (Logo Odfiz abstrak)
            let size = 100.0;
            let color = egui::Color32::from_rgb(0, 255, 127);
            
            painter.rect_filled(
                egui::Rect::from_center_size(center, egui::vec2(size, size)),
                10.0, // rounding
                color
            );

            // Minta layar refresh terus untuk animasi
            ctx.request_repaint();
        });
    }
}

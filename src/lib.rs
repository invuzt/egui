#![cfg(target_os = "android")]
use eframe::egui;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    // Seringkali FC terjadi karena driver GPU (WGPU). 
    // Kita coba paksa menggunakan GLOW (OpenGL) jika WGPU bermasalah.
    options.renderer = eframe::Renderer::Glow;

    let _ = eframe::run_native(
        "Odfiz App",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(3.0);
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(MyApp::default())
        }),
    );
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // Pastikan file font.ttf ada di folder assets yang sejajar dengan folder src
    let font_data = include_bytes!("../assets/font.ttf");

    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(font_data),
    );

    // Gunakan cara yang lebih aman untuk memasukkan font
    fonts.families.entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    fonts.families.entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);
}

struct MyApp {
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { text: "Odfiz Rust".to_owned() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.heading("🚀 Odfiz Native UI");
                ui.add_space(20.0);
                ui.text_edit_singleline(&mut self.text);
                ui.label(format!("Halo: {}", self.text));
            });
        });
    }
}

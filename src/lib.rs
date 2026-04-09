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
        "Zamera Alpha",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.8); 
            setup_custom_fonts(&cc.egui_ctx); // MUAT FONT DI SINI
            Box::new(ZameraApp::default())
        }),
    );
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Memasukkan file font ke dalam binary saat compile
    // Pastikan path ../assets/font.ttf benar!
    let font_data = include_bytes!("../assets/font.ttf");

    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(font_data),
    );

    // Prioritaskan font ini untuk teks biasa (Proportional)
    fonts.families.entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Dan untuk teks terminal/kode (Monospace)
    fonts.families.entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);
}

struct ZameraApp {
    zoom_level: f32,
    color: egui::Color32,
}

impl Default for ZameraApp {
    fn default() -> Self {
        Self {
            zoom_level: 0.3,
            color: egui::Color32::from_rgb(0, 255, 127),
        }
    }
}

impl eframe::App for ZameraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                // SEKARANG TEKS INI AKAN MUNCUL!
                ui.heading("ODFIZ ZAMERA"); 
                ui.label("Alpha Preview v0.1");

                let viewer_rect = ui.available_rect_before_wrap();
                ui.painter().rect_filled(
                    egui::Rect::from_center_size(viewer_rect.center(), egui::vec2(280.0, 280.0)),
                    10.0,
                    self.color.linear_multiply(0.2)
                );

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(30.0);
                    
                    // Label Slider
                    ui.label(format!("Zoom: {:.1}x", 1.0 + self.zoom_level * 4.0));
                    ui.add(egui::Slider::new(&mut self.zoom_level, 0.0..=1.0).show_value(false));
                    
                    ui.add_space(20.0);
                    
                    // Tombol Preset Warna
                    ui.horizontal(|ui| {
                        ui.add_space(ui.available_width()/2.0 - 45.0);
                        for (name, c) in [("G", egui::Color32::GREEN), ("R", egui::Color32::RED), ("B", egui::Color32::BLUE)] {
                            if ui.button(name).clicked() { self.color = c; }
                        }
                    });
                });
            });
        });
        ctx.request_repaint();
    }
}

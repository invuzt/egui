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
            // Kita coba muat, kalau gagal aplikasi tetap jalan (safe mode)
            let _ = setup_custom_fonts(&cc.egui_ctx); 
            Box::new(ZameraApp::default())
        }),
    );
}

fn setup_custom_fonts(ctx: &egui::Context) -> Option<()> {
    let mut fonts = egui::FontDefinitions::default();
    
    // Gunakan include_bytes! hanya jika filenya ada
    // Jika masih FC, pastikan ukuran file font Anda sekecil mungkin (<100KB)
    let font_data = include_bytes!("../assets/font.ttf");

    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(font_data),
    );

    fonts.families.get_mut(&egui::FontFamily::Proportional)?
        .insert(0, "my_font".to_owned());

    ctx.set_fonts(fonts);
    Some(())
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
                
                // Gunakan cara manual menggambar teks jika font engine gagal
                ui.heading("ODFIZ ZAMERA"); 

                let viewer_rect = ui.available_rect_before_wrap();
                ui.painter().rect_filled(
                    egui::Rect::from_center_size(viewer_rect.center(), egui::vec2(280.0, 280.0)),
                    10.0,
                    self.color.linear_multiply(0.2)
                );

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(30.0);
                    ui.add(egui::Slider::new(&mut self.zoom_level, 0.0..=1.0).show_value(false));
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        ui.add_space(ui.available_width()/2.0 - 45.0);
                        // Jika font gagal, tombol ini akan kosong/kotak-kotak, tapi tidak FC
                        if ui.button("G").clicked() { self.color = egui::Color32::GREEN; }
                        if ui.button("R").clicked() { self.color = egui::Color32::RED; }
                        if ui.button("B").clicked() { self.color = egui::Color32::BLUE; }
                    });
                });
            });
        });
        ctx.request_repaint();
    }
}

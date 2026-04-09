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
            setup_custom_fonts(&cc.egui_ctx);
            cc.egui_ctx.set_pixels_per_point(3.0);
            Box::new(MyApp::default())
        }),
    );
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Memasukkan font langsung ke dalam binary aplikasi
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
    );

    // Atur font sebagai prioritas utama untuk semua gaya tulisan
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
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
                ui.add_space(100.0);
                ui.heading("🚀 Odfiz Native UI");
                
                ui.add_space(20.0);
                ui.label("Jika tulisan ini muncul, font sukses!");
                
                ui.add_space(20.0);
                ui.text_edit_singleline(&mut self.text);
                
                if ui.button("KLIK KONFIRMASI").clicked() {
                    self.text = "Berhasil!".to_owned();
                }
            });
        });
    }
}

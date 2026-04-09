#![cfg(target_os = "android")]
use eframe::egui;

// DEFINISI WARNA (Sama persis dengan gambar)
const COL_BG: egui::Color32 = egui::Color32::from_rgb(10, 10, 10);      // Latar Belakang Total
const COL_CARD: egui::Color32 = egui::Color32::from_rgb(18, 18, 18);    // Kartu Utama
const COL_INNER: egui::Color32 = egui::Color32::from_rgb(26, 26, 26);   // Kartu Dalam
const COL_ACCENT: egui::Color32 = egui::Color32::from_rgb(255, 65, 90); // Merah Aksen/Heading
const COL_ONLINE: egui::Color32 = egui::Color32::from_rgb(0, 200, 110); // Hijau Online
const COL_TEXT_MAIN: egui::Color32 = egui::Color32::from_rgb(220, 220, 220); // Teks Putih Abu
const COL_TEXT_WEAK: egui::Color32 = egui::Color32::from_rgb(130, 130, 130); // Teks Lemah/Label

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Core",
        options,
        Box::new(|cc| {
            // Skala DPI diubah ke 2.2 agar muat dan terlihat proporsional di HP
            cc.egui_ctx.set_pixels_per_point(2.2); 
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(OdfizCore::default())
        }),
    );
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // Pastikan JetBrains Mono 137KB ada di assets/font.ttf
    let font_data = include_bytes!("../assets/font.ttf");

    fonts.font_data.insert(
        "jetbrains_mono".to_owned(),
        egui::FontData::from_static(font_data),
    );

    // Paksa semua gaya teks menggunakan JetBrains Mono
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "jetbrains_mono".to_owned());
    fonts.families.entry(egui::FontFamily::Monospace).or_default().push("jetbrains_mono".to_owned());

    ctx.set_fonts(fonts);
}

struct OdfizCore {
    // Statis saja untuk UI tiruan
}

impl Default for OdfizCore {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for OdfizCore {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // Atur Visuals Global (Dark Theme)
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(COL_TEXT_MAIN);
        ctx.set_visuals(visuals);

        // Panel Utama dengan Latar Gelap Gulita
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(COL_BG))
            .show(ctx, |ui| {
            
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 4.0); // Posisikan di tengah

                // 1. KARTU UTAMA (Container Besar)
                egui::Frame::none()
                    .fill(COL_CARD)
                    .rounding(20.0) // Sudut membulat
                    .inner_margin(egui::Margin::symmetric(25.0, 30.0)) // Padding dalam
                    .show(ui, |ui| {
                        
                        // HEADING: ODFIZ CORE (Teks Merah)
                        ui.heading(egui::RichText::new("ODFIZ CORE")
                            .color(COL_ACCENT)
                            .strong()
                            .letter_spacing(1.5));
                        
                        ui.add_space(15.0);

                        // 2. KARTU DALAM (Detail Panel)
                        egui::Frame::none()
                            .fill(COL_INNER)
                            .rounding(10.0)
                            .inner_margin(egui::Margin::symmetric(15.0, 15.0))
                            .show(ui, |ui| {
                                
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                    
                                    // Aksen Garis Merah di Kiri (Manual Draw)
                                    let line_start = ui.cursor().current_pos();
                                    let line_painter = ui.painter();
                                    line_painter.line_segment(
                                        [egui::pos2(line_start.x - 10.0, line_start.y), egui::pos2(line_start.x - 10.0, line_start.y + 60.0)],
                                        egui::Stroke::new(3.0, COL_ACCENT)
                                    );

                                    ui.add_space(5.0); // Jarak dari garis ke teks

                                    // Tata Letak Teks Detail
                                    ui.vertical(|ui| {
                                        ui.spacing_mut().item_spacing.y = 2.0;

                                        // BRAND
                                        ui.label(egui::RichText::new("BRAND").color(COL_TEXT_WEAK).size(11.0));
                                        ui.label(egui::RichText::new("Odfiz Tech").size(14.0));

                                        ui.add_space(3.0);

                                        // ENGINE
                                        ui.label(egui::RichText::new("ENGINE").color(COL_TEXT_WEAK).size(11.0));
                                        ui.label(egui::RichText::new("Rust Axum 0.7").size(14.0));

                                        ui.add_space(3.0);

                                        // STATUS
                                        ui.label(egui::RichText::new("STATUS").color(COL_TEXT_WEAK).size(11.0));
                                        ui.label(egui::RichText::new("Online").color(COL_ONLINE).strong());
                                    });
                                });
                            });

                        ui.add_space(20.0);

                        // 3. TOMBOL REFRESH (Aksen Merah Penuh)
                        let button_text = egui::RichText::new(" REFRESH ").strong();
                        let btn_frame = egui::Frame::none()
                            .fill(COL_ACCENT)
                            .rounding(10.0)
                            .inner_margin(egui::Margin::symmetric(ui.available_width()/2.0 - 45.0, 10.0));
                        
                        ui.scope_builder(egui::UiBuilder::new(), |ui| {
                            ui.set_visuals(egui::Visuals {
                                override_text_color: Some(egui::Color32::WHITE),
                                ..Default::default()
                            });
                            
                            // Kita buat tombol kustom agar warnanya statis
                            let button = egui::Button::new(button_text).frame(true);
                            ui.style_mut().visuals.widgets.inactive.bg_fill = COL_ACCENT;
                            ui.style_mut().visuals.widgets.hovered.bg_fill = COL_ACCENT.linear_multiply(0.8);
                            
                            ui.add_sized(egui::vec2(ui.available_width(), 40.0), button);
                        });
                    });
            });
        });
        ctx.request_repaint();
    }
}

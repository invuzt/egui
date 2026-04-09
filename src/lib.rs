#![cfg(target_os = "android")]
use eframe::egui;

const COL_BG: egui::Color32 = egui::Color32::from_rgb(10, 10, 10);
const COL_CARD: egui::Color32 = egui::Color32::from_rgb(18, 18, 18);
const COL_INNER: egui::Color32 = egui::Color32::from_rgb(26, 26, 26);
const COL_ACCENT: egui::Color32 = egui::Color32::from_rgb(255, 65, 90);
const COL_ONLINE: egui::Color32 = egui::Color32::from_rgb(0, 200, 110);
const COL_TEXT_MAIN: egui::Color32 = egui::Color32::from_rgb(220, 220, 220);
const COL_TEXT_WEAK: egui::Color32 = egui::Color32::from_rgb(130, 130, 130);

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
            // DIET SKALA: Turun ke 1.6 - 1.7 agar layout terlihat lebih padat/compact
            cc.egui_ctx.set_pixels_per_point(1.7); 
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(OdfizCore::default())
        }),
    );
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let font_data = include_bytes!("../assets/font.ttf");
    fonts.font_data.insert("jb".to_owned(), egui::FontData::from_static(font_data));
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "jb".to_owned());
    ctx.set_fonts(fonts);
}

struct OdfizCore {}
impl Default for OdfizCore { fn default() -> Self { Self {} } }

impl eframe::App for OdfizCore {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(COL_TEXT_MAIN);
        ctx.set_visuals(visuals);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(COL_BG))
            .show(ctx, |ui| {
            
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 5.0); // Naikkan sedikit posisinya

                egui::Frame::none()
                    .fill(COL_CARD)
                    .rounding(16.0) // Rounding lebih kecil supaya lebih sleek
                    .inner_margin(egui::Margin::symmetric(20.0, 25.0))
                    .show(ui, |ui| {
                        
                        ui.heading(egui::RichText::new("ODFIZ CORE")
                            .color(COL_ACCENT)
                            .strong()
                            .size(18.0) // Ukuran font heading diperkecil
                            .extra_letter_spacing(2.0));
                        
                        ui.add_space(12.0);

                        egui::Frame::none()
                            .fill(COL_INNER)
                            .rounding(8.0)
                            .inner_margin(egui::Margin::symmetric(12.0, 12.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let line_start = ui.cursor().min;
                                    ui.painter().line_segment(
                                        [egui::pos2(line_start.x - 4.0, line_start.y), egui::pos2(line_start.x - 4.0, line_start.y + 55.0)],
                                        egui::Stroke::new(2.5, COL_ACCENT)
                                    );
                                    ui.add_space(8.0);

                                    ui.vertical(|ui| {
                                        ui.spacing_mut().item_spacing.y = 1.0;
                                        ui.label(egui::RichText::new("BRAND").color(COL_TEXT_WEAK).size(9.0));
                                        ui.label(egui::RichText::new("Odfiz Tech").size(13.0));
                                        ui.add_space(2.0);
                                        ui.label(egui::RichText::new("ENGINE").color(COL_TEXT_WEAK).size(9.0));
                                        ui.label(egui::RichText::new("Rust Axum 0.7").size(13.0));
                                        ui.add_space(2.0);
                                        ui.label(egui::RichText::new("STATUS").color(COL_TEXT_WEAK).size(9.0));
                                        ui.label(egui::RichText::new("Online").color(COL_ONLINE).size(13.0).strong());
                                    });
                                });
                            });

                        ui.add_space(15.0);
                        
                        // Tombol REFRESH yang lebih slim
                        ui.visuals_mut().widgets.inactive.bg_fill = COL_ACCENT;
                        ui.visuals_mut().widgets.hovered.bg_fill = COL_ACCENT;
                        ui.visuals_mut().widgets.active.bg_fill = COL_ACCENT;
                        
                        let button = egui::Button::new(
                            egui::RichText::new("REFRESH").color(egui::Color32::WHITE).size(12.0).strong()
                        ).min_size(egui::vec2(140.0, 32.0)); // Lebar dibatasi, tidak full width raksasa

                        ui.add(button);
                    });
            });
        });
        ctx.request_repaint();
    }
}

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
            cc.egui_ctx.set_pixels_per_point(2.2); 
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(OdfizCore::default())
        }),
    );
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let font_data = include_bytes!("../assets/font.ttf");
    fonts.font_data.insert(
        "jetbrains_mono".to_owned(),
        egui::FontData::from_static(font_data),
    );
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "jetbrains_mono".to_owned());
    fonts.families.entry(egui::FontFamily::Monospace).or_default().push("jetbrains_mono".to_owned());
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
                ui.add_space(ui.available_height() / 4.0);

                egui::Frame::none()
                    .fill(COL_CARD)
                    .rounding(20.0)
                    .inner_margin(egui::Margin::symmetric(25.0, 30.0))
                    .show(ui, |ui| {
                        
                        // Fix: Pakai extra_letter_spacing sesuai anjuran compiler
                        ui.heading(egui::RichText::new("ODFIZ CORE")
                            .color(COL_ACCENT)
                            .strong()
                            .extra_letter_spacing(1.5));
                        
                        ui.add_space(15.0);

                        egui::Frame::none()
                            .fill(COL_INNER)
                            .rounding(10.0)
                            .inner_margin(egui::Margin::symmetric(15.0, 15.0))
                            .show(ui, |ui| {
                                
                                ui.horizontal(|ui| {
                                    // Fix: Cara ambil posisi kursor yang benar di v0.27
                                    let line_start = ui.cursor().min;
                                    let line_painter = ui.painter();
                                    line_painter.line_segment(
                                        [egui::pos2(line_start.x - 5.0, line_start.y), egui::pos2(line_start.x - 5.0, line_start.y + 65.0)],
                                        egui::Stroke::new(3.0, COL_ACCENT)
                                    );

                                    ui.add_space(5.0);

                                    ui.vertical(|ui| {
                                        ui.spacing_mut().item_spacing.y = 2.0;
                                        ui.label(egui::RichText::new("BRAND").color(COL_TEXT_WEAK).size(11.0));
                                        ui.label(egui::RichText::new("Odfiz Tech").size(14.0));
                                        ui.add_space(3.0);
                                        ui.label(egui::RichText::new("ENGINE").color(COL_TEXT_WEAK).size(11.0));
                                        ui.label(egui::RichText::new("Rust Axum 0.7").size(14.0));
                                        ui.add_space(3.0);
                                        ui.label(egui::RichText::new("STATUS").color(COL_TEXT_WEAK).size(11.0));
                                        ui.label(egui::RichText::new("Online").color(COL_ONLINE).strong());
                                    });
                                });
                            });

                        ui.add_space(20.0);

                        // Fix: Pengganti scope_builder yang tidak ada di v0.27
                        let btn_color = COL_ACCENT;
                        ui.visuals_mut().widgets.inactive.bg_fill = btn_color;
                        ui.visuals_mut().widgets.hovered.bg_fill = btn_color;
                        ui.visuals_mut().widgets.active.bg_fill = btn_color;
                        
                        let button = egui::Button::new(
                            egui::RichText::new("REFRESH")
                                .color(egui::Color32::WHITE)
                                .strong()
                        ).min_size(egui::vec2(ui.available_width(), 40.0));

                        if ui.add(button).clicked() {
                            // Logic refresh di sini
                        }
                    });
            });
        });
        ctx.request_repaint();
    }
}

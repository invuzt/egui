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
        "Odfiz Graphic UI",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(3.0);
            Box::new(MyApp::default())
        }),
    );
}

struct MyApp {
    value: f32,
    checked: bool,
    color: egui::Color32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            value: 0.5, 
            checked: false,
            color: egui::Color32::from_rgb(0, 255, 127),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);

                // 1. SLIDER (Tanpa Label Teks)
                ui.add(egui::Slider::new(&mut self.value, 0.0..=1.0).show_value(false));
                
                ui.add_space(20.0);

                // 2. CHECKBOX / TOGGLE (Custom Visual)
                let rect = ui.allocate_exact_size(egui::vec2(60.0, 30.0), egui::Sense::click()).0;
                if ui.rect_contains_pointer(rect) && ui.input(|i| i.pointer.any_click()) {
                    self.checked = !self.checked;
                }
                let toggle_col = if self.checked { egui::Color32::LIGHT_BLUE } else { egui::Color32::GRAY };
                ui.painter().rect_filled(rect, 15.0, toggle_col);

                ui.add_space(20.0);

                // 3. COLOR PICKER (Sangat visual, minim teks)
                ui.color_edit_button_srgba(&mut self.color);

                ui.add_space(40.0);

                // 4. DYNAMIC GRAPHIC (Viewer)
                // Menggunakan value dari slider untuk mengubah ukuran objek
                let painter = ui.painter();
                let center = ui.max_rect().center();
                let dynamic_size = 50.0 + (self.value * 100.0);
                
                painter.circle_filled(
                    center, 
                    dynamic_size, 
                    self.color
                );

                // Garis dekoratif (Bezier Curve)
                painter.error_path(
                    vec![
                        center + egui::vec2(-100.0, 100.0),
                        center + egui::vec2(0.0, 150.0),
                        center + egui::vec2(100.0, 100.0),
                    ],
                    egui::Color32::WHITE,
                    2.0
                );
            });
        });
        ctx.request_repaint();
    }
}

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
            value: 0.3, 
            checked: true,
            color: egui::Color32::from_rgb(0, 255, 127),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let center = ui.max_rect().center();
            
            // --- AREA VISUAL (Latar Belakang) ---
            let painter = ui.painter();
            let dynamic_size = 30.0 + (self.value * 150.0);
            
            painter.circle_filled(center, dynamic_size, self.color);
            
            if self.checked {
                painter.circle_stroke(center, dynamic_size + 10.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
            }

            // --- AREA KONTROL (Diatur ke Bawah Layar) ---
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(40.0); // Margin bawah

                // Row untuk Color Picker dan Toggle
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() / 4.0);
                    ui.color_edit_button_srgba(&mut self.color);
                    
                    ui.add_space(20.0);
                    
                    let (rect, response) = ui.allocate_exact_size(egui::vec2(60.0, 30.0), egui::Sense::click());
                    if response.clicked() { self.checked = !self.checked; }
                    let toggle_col = if self.checked { egui::Color32::LIGHT_BLUE } else { egui::Color32::GRAY };
                    ui.painter().rect_filled(rect, 15.0, toggle_col);
                });

                ui.add_space(20.0);

                // Slider (Full width di bawah)
                ui.add(egui::Slider::new(&mut self.value, 0.0..=1.0)
                    .show_value(false)
                    .trailing_fill(true));

                ui.add_space(20.0);
            });
        });
        ctx.request_repaint();
    }
}

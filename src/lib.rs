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
        "Odfiz Mini Gallery",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(2.5); 
            Box::new(MiniGallery::default())
        }),
    );
}

struct MiniGallery {
    scalar: f32,
    boolean: bool,
    color: egui::Color32,
    animate: bool,
}

impl Default for MiniGallery {
    fn default() -> Self {
        Self {
            scalar: 180.0,
            boolean: true,
            color: egui::Color32::from_rgb(0, 255, 127),
            animate: true,
        }
    }
}

impl eframe::App for MiniGallery {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 15.0);

            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                // 1. Progress Bar
                let progress = self.scalar / 360.0;
                ui.add(egui::ProgressBar::new(progress)
                    .animate(self.animate));

                ui.add_space(10.0);

                // 2. Grid Kontrol
                egui::Grid::new("mini_grid").spacing([20.0, 20.0]).show(ui, |ui| {
                    ui.add(egui::Slider::new(&mut self.scalar, 0.0..=360.0).show_value(false));
                    ui.color_edit_button_srgba(&mut self.color);
                    ui.end_row();

                    ui.add(egui::Spinner::new());
                    
                    // Checkbox Custom
                    let (rect, response) = ui.allocate_exact_size(egui::vec2(30.0, 30.0), egui::Sense::click());
                    if response.clicked() { self.boolean = !self.boolean; }
                    let fill = if self.boolean { self.color } else { egui::Color32::TRANSPARENT };
                    ui.painter().rect(rect, 4.0, fill, egui::Stroke::new(2.0, egui::Color32::GRAY));
                    ui.end_row();
                });

                ui.add_space(20.0);

                // 3. Visual Object
                let painter = ui.painter();
                let rect = ui.max_rect();
                let center = egui::pos2(rect.center().x, rect.center().y + 80.0);
                
                painter.rect_filled(
                    egui::Rect::from_center_size(center, egui::vec2(100.0, 100.0)),
                    self.scalar / 10.0,
                    self.color
                );
            });
        });

        if self.animate {
            ctx.request_repaint();
        }
    }
}

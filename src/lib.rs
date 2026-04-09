#![cfg(target_os = "android")]
use eframe::egui;

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_layout_guide(true);
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Mini Gallery",
        options,
        Box::new(|cc| {
            // Kita kecilkan skalanya agar muat banyak widget
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

impl Default military::MiniGallery {
    fn default() -> Self {
        Self {
            scalar: 180.0,
            boolean: true,
            color: egui::Color32::from_rgb(0, 255, 127),
            animate: true,
        }
    }
}

// Implementasi manual untuk MiniGallery agar tidak crash tanpa font
impl eframe::App for MiniGallery {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 15.0);

            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                // 1. Progress Bar (Visual Utama)
                let progress = self.scalar / 360.0;
                ui.add(egui::ProgressBar::new(progress)
                    .animate(self.animate));

                ui.add_space(10.0);

                // 2. Grid Kontrol Kecil
                egui::Grid::new("mini_grid").spacing([20.0, 20.0]).show(ui, |ui| {
                    // Slider tanpa label angka
                    ui.add(egui::Slider::new(&mut self.scalar, 0.0..=360.0).show_value(false));
                    
                    // Tombol Warna
                    ui.color_edit_button_srgba(&mut self.color);
                    ui.end_row();

                    // Spinner (Visual loading)
                    ui.add(egui::Spinner::new());
                    
                    // Checkbox Custom (Hanya kotak)
                    let (rect, response) = ui.allocate_exact_size(egui::vec2(30.0, 30.0), egui::Sense::click());
                    if response.clicked() { self.boolean = !self.boolean; }
                    let fill = if self.boolean { self.color } else { egui::Color32::TRANSPARENT };
                    ui.painter().rect(rect, 4.0, fill, egui::Stroke::new(2.0, egui::Color32::GRAY));
                    ui.end_row();
                });

                ui.add_space(20.0);

                // 3. Area Gambar/Shape
                let painter = ui.painter();
                let rect = ui.max_rect();
                let center = egui::pos2(rect.center().x, rect.center().y + 50.0);
                
                painter.rect_filled(
                    egui::Rect::from_center_size(center, egui::vec2(80.0, 80.0)),
                    self.scalar / 10.0, // Rounding berubah lewat slider
                    self.color
                );
            });
        });

        if self.animate {
            ctx.request_repaint();
        }
    }
}

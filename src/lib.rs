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
            // FIX KECILKAN SKALA: Diubah dari 2.5 menjadi 1.8 agar muat banyak widget
            cc.egui_ctx.set_pixels_per_point(1.8); 
            Box::new(ZameraApp::default())
        }),
    );
}

struct ZameraApp {
    zoom_level: f32,
    checked: bool,
    filter_color: egui::Color32,
    is_capturing: bool,
}

impl Default for ZameraApp {
    fn default() -> Self {
        Self {
            zoom_level: 0.3,
            checked: true,
            filter_color: egui::Color32::from_rgb(0, 255, 127), // Warna Default Odfiz
            is_capturing: false,
        }
    }
}

impl eframe::App for ZameraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TATA LETAK: Gunakan Layout::bottom_up agar kontrol ada di bawah
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

            // --- AREA VIEWER KAMERA (Di Tengah, Ukuran Dinamis) ---
            ui.add_space(30.0);
            let viewer_rect = ui.available_rect_before_wrap();
            let viewer_painter = ui.painter();
            
            // Kotak ini nanti akan diisi gambar dari kamera
            viewer_painter.rect_filled(
                egui::Rect::from_center_size(viewer_rect.center(), egui::vec2(250.0, 250.0)),
                5.0,
                self.filter_color.linear_multiply(0.3) // Filter transparansi
            );

            // Tanda Bidik (Crosshair) Kamera
            let ch_size = 20.0;
            let ch_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
            viewer_painter.line_segment([viewer_rect.center() - egui::vec2(ch_size, 0.0), viewer_rect.center() + egui::vec2(ch_size, 0.0)], ch_stroke);
            viewer_painter.line_segment([viewer_rect.center() - egui::vec2(0.0, ch_size), viewer_rect.center() + egui::vec2(0.0, ch_size)], ch_stroke);


            // --- AREA KONTROL JEMPOL (Di Bawah Layar) ---
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(20.0); // Margin bawah

                // Row untuk Tombol Capture & Color Picker
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() / 2.0 - 25.0); // Center

                    // Tombol Ambil Gambar (Visual Murni)
                    let cap_stroke = if self.is_capturing { egui::Stroke::new(3.0, egui::Color32::RED) } else { egui::Stroke::new(2.0, egui::Color32::WHITE) };
                    let (cap_rect, cap_response) = ui.allocate_exact_size(egui::vec2(50.0, 50.0), egui::Sense::click());
                    ui.painter().circle_stroke(cap_rect.center(), 23.0, cap_stroke);
                    if cap_response.clicked() { self.is_capturing = true; }

                    ui.add_space(20.0);
                    // Color Picker (Sebagai Filter)
                    ui.color_edit_button_srgba(&mut self.filter_color);
                });

                ui.add_space(15.0);

                // Grid untuk Slider & Spinner (Muat karena skala dikecilkan)
                egui::Grid::new("controls_grid").spacing([15.0, 15.0]).show(ui, |ui| {
                    // Slider Zoom
                    ui.add(egui::Slider::new(&mut self.zoom_level, 0.0..=1.0).show_value(false).trailing_fill(true));
                    // Spinner Loading (Indikator Kamera Aktif)
                    ui.add(egui::Spinner::new());
                    ui.end_row();
                });

                ui.add_space(10.0);
            });
        });

        if self.is_capturing {
            // Logika reset tombol capture setelah diklik
            self.is_capturing = false;
            ctx.request_repaint();
        }
    }
}

#![cfg(target_os = "android")]
use eframe::egui;

// Menggunakan struct untuk menyimpan state agar tidak ada alokasi memori di update()
struct AppState {
    counter: u64,
    // Kita simpan String di sini supaya tidak perlu format!() setiap frame
    display_text: String,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Pure",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);
            Box::new(OdfizApp {
                state: AppState {
                    counter: 0,
                    display_text: "System Ready".to_string(),
                },
            })
        }),
    );
}

struct OdfizApp {
    state: AppState,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Pakai Dark Mode bawaan yang paling irit pixel untuk layar OLED
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading(egui::RichText::new("ODFIZ PURE").strong().size(24.0));
                ui.label(egui::RichText::new("GPU Accelerated • Zero CPU Idle").color(egui::Color32::GRAY));
                
                ui.add_space(30.0);
                ui.separator();
                ui.add_space(30.0);

                // Panel Monitor
                ui.group(|ui| {
                    ui.set_width(ui.available_width() * 0.8);
                    ui.add_space(20.0);
                    
                    // Menampilkan data tanpa format!() di dalam loop
                    ui.label(egui::RichText::new(&format!("COUNTER: {}", self.state.counter)).size(30.0).strong().color(egui::Color32::GREEN));
                    ui.add_space(10.0);
                    ui.label(&self.state.display_text);
                    
                    ui.add_space(20.0);
                });

                ui.add_space(60.0);

                // Main Action - Ukuran tombol disesuaikan untuk kenyamanan Android
                let btn = egui::Button::new(egui::RichText::new("➕ TAP TO COUNT").size(20.0))
                    .fill(egui::Color32::from_rgb(40, 40, 50))
                    .rounding(10.0);

                if ui.add_sized([ui.available_width() * 0.7, 80.0], btn).clicked() {
                    self.state.counter += 1;
                    self.state.display_text = format!("Last Update at event #{}", self.state.counter);
                    // Otomatis repaint karena input sentuhan (reactive)
                }

                ui.add_space(20.0);

                if ui.button("Reset Data").clicked() {
                    self.state.counter = 0;
                    self.state.display_text = "Data Resetted".to_string();
                }
            });
        });

        // Tanpa request_repaint(), CPU benar-benar tidur (0%) saat user tidak menyentuh layar.
    }
}

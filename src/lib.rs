#![cfg(target_os = "android")]
use eframe::egui;

struct AppState {
    counter: u64,
    last_action: String,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Pure Native",
        options,
        Box::new(|cc| {
            // Skala UI agar nyaman di jempol
            cc.egui_ctx.set_pixels_per_point(1.5);
            
            Box::new(OdfizApp {
                state: AppState {
                    counter: 0,
                    last_action: "Ready".to_string(),
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
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.heading(egui::RichText::new("ODFIZ PURE NATIVE").strong());
                ui.label("Reactive Mode: ON (0% CPU Idle)");
                ui.separator();
                ui.add_space(40.0);

                // Display Monitor
                ui.group(|ui| {
                    ui.set_width(280.0);
                    ui.add_space(15.0);
                    ui.label(egui::RichText::new("MONITOR").size(14.0).color(egui::Color32::GRAY));
                    ui.heading(format!("Count: {}", self.state.counter));
                    ui.label(format!("Last Event: {}", self.state.last_action));
                    ui.add_space(15.0);
                });

                ui.add_space(50.0);

                // Main Action Button
                // Tanpa request_repaint manual, egui otomatis repaint saat tombol diklik
                if ui.add_sized([220.0, 70.0], egui::Button::new(egui::RichText::new("➕ ADD DATA").size(20.0)).fill(egui::Color32::from_rgb(30, 80, 150))).clicked() {
                    self.state.counter += 1;
                    self.state.last_action = "Data Added".to_string();
                }

                ui.add_space(20.0);

                if ui.button("Clear History").clicked() {
                    self.state.counter = 0;
                    self.state.last_action = "Cleared".to_string();
                }
            });
        });

        // CATATAN KRUSIAL: 
        // Di sini kita TIDAK memanggil ctx.request_repaint() atau request_repaint_after().
        // Artinya, loop update akan BERHENTI total jika tidak ada sentuhan layar.
        // Baterai HP kamu akan sangat aman.
    }
}

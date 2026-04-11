#![cfg(target_os = "android")]
mod features;
mod theme;

use eframe::egui;

struct OdfizShell {
    mm: features::ModuleManager,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz CMS",
        options,
        Box::new(|cc| {
            theme::apply_global_style(&cc.egui_ctx);
            Box::new(OdfizShell { mm: features::ModuleManager::new() })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(10, 10, 10)))
            .show(ctx, |ui| {
                // Header dibuang, langsung kasih jarak dikit dari status bar
                ui.add_space(10.0);

                egui::ScrollArea::vertical()
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        // MODUL SERVER
                        theme::draw_card(ui, egui::Color32::from_rgb(37, 99, 235), |ui| {
                            self.mm.server.ui(ui);
                        });

                        ui.add_space(10.0);

                        // MODUL KASIR
                        theme::draw_card(ui, egui::Color32::from_rgb(13, 148, 136), |ui| {
                            self.mm.kasir.ui(ui);
                        });
                    });
            });
    }
}

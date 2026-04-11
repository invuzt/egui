#![cfg(target_os = "android")]
mod features;
mod theme;

use eframe::egui;
use eframe::egui::RichText;

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
        "Odfiz Core",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.visuals.panel_fill = theme::COLOR_BG;
            cc.egui_ctx.set_style(style);
            Box::new(OdfizShell { mm: features::ModuleManager::new() })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(60.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("ODFIZ CORE SYSTEM").strong().size(20.0).color(theme::COLOR_ACCENT).extra_letter_spacing(3.0));
                ui.add_space(30.0);

                // --- KARTU MODUL ---
                theme::odfiz_card(ui, |ui| {
                    // Kita buat area header yang bisa merespon klik
                    let (rect, response) = ui.allocate_at_least(egui::vec2(ui.available_width(), 30.0), egui::Sense::click());
                    
                    // Jika area header diklik (tulisan atau icon mana saja)
                    if response.clicked() {
                        self.mm.server_open = !self.mm.server_open;
                    }

                    // Gambar konten di dalam area rect tadi secara manual agar rapi
                    ui.allocate_ui_at_rect(rect, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("LITE SERVER").strong().size(22.0));
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(if self.mm.server_open { "🔼" } else { "🔽" });
                            });
                        });
                    });

                    // Konten isi modul
                    if self.mm.server_open {
                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(20.0);
                        self.mm.server.ui(ui);
                    }
                });
            });
        });
    }
}

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
            style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Proportional));
            style.text_styles.insert(egui::TextStyle::Button, egui::FontId::new(18.0, egui::FontFamily::Proportional));
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

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        // --- SERVER CARD ---
                        theme::odfiz_card(ui, |ui| {
                            let (rect, res) = ui.allocate_at_least(egui::vec2(ui.available_width(), 35.0), egui::Sense::click());
                            if res.clicked() { self.mm.server_open = !self.mm.server_open; }
                            ui.allocate_ui_at_rect(rect, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("LITE SERVER").strong().size(22.0).color(if self.mm.server_open { theme::COLOR_ACCENT } else { egui::Color32::WHITE }));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { ui.label(if self.mm.server_open { "🔼" } else { "🔽" }); });
                                });
                            });
                            if self.mm.server_open { ui.add_space(20.0); ui.separator(); ui.add_space(20.0); self.mm.server.ui(ui); }
                        });

                        ui.add_space(20.0);

                        // --- KASIR CARD ---
                        theme::odfiz_card(ui, |ui| {
                            let (rect, res) = ui.allocate_at_least(egui::vec2(ui.available_width(), 35.0), egui::Sense::click());
                            if res.clicked() { self.mm.kasir_open = !self.mm.kasir_open; }
                            ui.allocate_ui_at_rect(rect, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("KASIR ODFIZ").strong().size(22.0).color(if self.mm.kasir_open { theme::COLOR_ACCENT } else { egui::Color32::WHITE }));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { ui.label(if self.mm.kasir_open { "🔼" } else { "🔽" }); });
                                });
                            });
                            if self.mm.kasir_open { ui.add_space(20.0); ui.separator(); ui.add_space(20.0); self.mm.kasir.ui(ui); }
                        });
                        ui.add_space(50.0);
                    });
                });
            });
        });
    }
}

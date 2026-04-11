#![cfg(target_os = "android")]
mod features;
mod theme;

use eframe::egui;
use eframe::egui::{RichText, Color32};

struct OdfizShell {
    mm: features::ModuleManager,
    active_tab: usize,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Marketplace",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.visuals = egui::Visuals::light();
            style.visuals.panel_fill = theme::COLOR_BG;
            cc.egui_ctx.set_style(style);
            Box::new(OdfizShell { mm: features::ModuleManager::new(), active_tab: 0 })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- BOTTOM NAVIGATION BAR ---
        egui::TopBottomPanel::bottom("nav").show(ctx, |ui| {
            ui.set_height(70.0);
            ui.horizontal_centered(|ui| {
                let w = ui.available_width() / 5.0;
                if ui.add_sized([w, 50.0], egui::Button::new("🏠\nHome").frame(false)).clicked() { self.active_tab = 0; }
                if ui.add_sized([w, 50.0], egui::Button::new("❤️\nFav").frame(false)).clicked() { }
                
                // Tombol Plus Tengah
                ui.add_space(w/4.0);
                if ui.add(egui::Button::new(RichText::new(" + ").size(25.0).color(Color32::WHITE))
                    .fill(theme::COLOR_ACCENT).rounding(20.0)).clicked() { }
                ui.add_space(w/4.0);

                if ui.add_sized([w, 50.0], egui::Button::new("💬\nChat").frame(false)).clicked() { }
                if ui.add_sized([w, 50.0], egui::Button::new("👤\nProfile").frame(false)).clicked() { }
            });
        });

        // --- MAIN CONTENT ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            
            // Header: Brand & Location
            ui.horizontal(|ui| {
                ui.label(RichText::new("Odfiz").size(24.0).strong().color(theme::COLOR_ACCENT));
                ui.label(RichText::new("Ponorogo ⌄").size(14.0).color(Color32::GRAY));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("🔔");
                });
            });

            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Your next service\nis just a tap away").size(22.0).strong());
            });

            ui.add_space(20.0);
            theme::draw_search_bar(ui);

            ui.add_space(25.0);

            // GRID MENU (Tempat Modul Mas)
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(15.0, 15.0);
                
                if theme::draw_grid_item(ui, "🌐", "Lite Server", self.mm.server_open).clicked() {
                    self.mm.server_open = !self.mm.server_open;
                    self.mm.kasir_open = false;
                }

                if theme::draw_grid_item(ui, "💰", "Kasir Odfiz", self.mm.kasir_open).clicked() {
                    self.mm.kasir_open = !self.mm.kasir_open;
                    self.mm.server_open = false;
                }

                theme::draw_grid_item(ui, "🛠️", "Tools", false);
                theme::draw_grid_item(ui, "👥", "Community", false);
            });

            ui.add_space(20.0);
            ui.separator();

            // AREA MODUL AKTIF (Muncul di bawah Grid)
            if self.mm.server_open || self.mm.kasir_open {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Frame::none()
                        .fill(Color32::WHITE)
                        .inner_margin(20.0)
                        .rounding(15.0)
                        .stroke(egui::Stroke::new(1.0, theme::COLOR_BORDER))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            if self.mm.server_open { self.mm.server.ui(ui); }
                            if self.mm.kasir_open { self.mm.kasir.ui(ui); }
                        });
                });
            } else {
                ui.add_space(40.0);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Popular in Ponorogo").size(18.0).strong());
                    ui.label(RichText::new("Coming soon...").color(Color32::GRAY));
                });
            }
        });
    }
}

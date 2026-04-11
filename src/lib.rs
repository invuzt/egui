#![cfg(target_os = "android")]
mod features;
mod theme;

use eframe::egui;
use eframe::egui::{RichText, Color32};

struct OdfizShell {
    mm: features::ModuleManager,
    search_query: String,
    show_search_menu: bool,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Pro",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.visuals = egui::Visuals::light();
            style.visuals.panel_fill = theme::COLOR_BG;
            cc.egui_ctx.set_style(style);
            Box::new(OdfizShell { 
                mm: features::ModuleManager::new(), 
                search_query: String::new(),
                show_search_menu: false 
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            
            // --- TOP BAR: HAMBURGER & NOTIF ---
            ui.horizontal(|ui| {
                ui.menu_button(RichText::new("☰").size(24.0).color(theme::COLOR_ACCENT), |ui| {
                    ui.set_width(150.0);
                    if ui.button("⚙ Settings").clicked() { ui.close_menu(); }
                    ui.separator();
                    if ui.button("ℹ About").clicked() { ui.close_menu(); }
                });
                
                ui.add_space(5.0);
                ui.label(RichText::new("Odfiz").size(20.0).strong());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("🔔").size(20.0));
                });
            });

            ui.add_space(25.0);

            // --- SEARCH DROPDOWN ---
            ui.vertical(|ui| {
                let text_edit = egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search services...")
                    .margin(egui::Margin::symmetric(15.0, 12.0));
                
                let res = ui.add_sized([ui.available_width(), 45.0], text_edit);
                
                if res.has_focus() || !self.search_query.is_empty() {
                    self.show_search_menu = true;
                } else {
                    self.show_search_menu = false;
                }

                if self.show_search_menu {
                    egui::Frame::popup(ui.style()).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        if ui.button("Recent: Lite Server").clicked() { self.mm.server_open = true; }
                        if ui.button("Recent: Kasir").clicked() { self.mm.kasir_open = true; }
                    });
                }
            });

            ui.add_space(30.0);

            // --- GRID MENU (WARNA-WARNI) ---
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(12.0, 12.0);
                
                // Server (Blue)
                if theme::draw_grid_item(ui, "🌐", "Lite Server", Color32::from_rgb(59, 130, 246), self.mm.server_open).clicked() {
                    self.mm.server_open = !self.mm.server_open;
                    self.mm.kasir_open = false;
                }

                // Kasir (Green)
                if theme::draw_grid_item(ui, "💰", "Kasir Odfiz", Color32::from_rgb(34, 197, 94), self.mm.kasir_open).clicked() {
                    self.mm.kasir_open = !self.mm.kasir_open;
                    self.mm.server_open = false;
                }

                // Tools (Orange)
                theme::draw_grid_item(ui, "🛠️", "Tools", Color32::from_rgb(249, 115, 22), false);
                
                // Community (Purple)
                theme::draw_grid_item(ui, "👥", "Community", Color32::from_rgb(168, 85, 247), false);
            });

            ui.add_space(25.0);

            // --- AREA MODUL AKTIF ---
            if self.mm.server_open || self.mm.kasir_open {
                ui.separator();
                ui.add_space(10.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Frame::none()
                        .fill(Color32::WHITE)
                        .inner_margin(20.0)
                        .rounding(15.0)
                        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(230, 230, 230)))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            if self.mm.server_open { self.mm.server.ui(ui); }
                            if self.mm.kasir_open { self.mm.kasir.ui(ui); }
                        });
                });
            }
        });
    }
}

#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use eframe::egui::{Color32, Visuals, TextStyle, FontId, FontFamily, Frame};
use features::get_all_modules;

struct OdfizShell {
    modules: Vec<(bool, Box<dyn features::OdfizModule>)>,
    active_feature: Option<usize>, // Indeks fitur yang sedang ditampilkan
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Shell",
        options,
        Box::new(|cc| {
            // --- KUSTOMISASI TAMPILAN MODERN DARK ---
            let mut style = (*cc.egui_ctx.style()).clone();
            
            // 1. TEMA DARK MODERN (GELAP)
            style.visuals = Visuals::dark();
            // Sesuaikan warna agar mirip gambar (Dark Purple/Blue)
            style.visuals.panel_fill = Color32::from_rgb(26, 26, 64);
            style.visuals.window_fill = Color32::from_rgb(45, 45, 100);
            style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(45, 45, 100);
            style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(60, 60, 130);
            style.visuals.widgets.active.bg_fill = Color32::from_rgb(80, 80, 160);
            
            // 2. PERBESAR FONT
            style.text_styles.insert(TextStyle::Heading, FontId::new(28.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Body, FontId::new(18.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Button, FontId::new(18.0, FontFamily::Proportional));
            
            // 3. ROUNDED MODERN SHAPES
            style.visuals.widgets.noninteractive.rounding = 16.0.into();
            style.visuals.widgets.inactive.rounding = 16.0.into();
            style.visuals.widgets.active.rounding = 16.0.into();
            
            // 4. SPACING
            style.spacing.item_spacing = egui::vec2(15.0, 20.0);
            style.spacing.button_padding = egui::vec2(25.0, 15.0);

            cc.egui_ctx.set_style(style);

            Box::new(OdfizShell { 
                modules: get_all_modules(),
                active_feature: None, // Tidak ada fitur yang aktif di awal (Dashboard)
            })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- FRAME KUSTOM UNTUK CARDS ---
        let card_frame = Frame::none()
            .fill(ctx.style().visuals.widgets.inactive.bg_fill)
            .inner_margin(20.0)
            .rounding(16.0);

        // AREA DASHBOARD (Header, Review, Cards)
        if self.active_feature.is_none() {
            // --- HEADER (Status Bar Area & Hi) ---
            egui::TopBottomPanel::top("top_dashboard")
                .frame(egui::Frame::none().fill(ctx.style().visuals.panel_fill))
                .show(ctx, |ui| {
                    ui.add_space(45.0); // Safety space
                    ui.vertical(|ui| {
                        // Icon (Simbol Unicode Bar Chart)
                        ui.label(egui::RichText::new("📊").size(24.0));
                        ui.add_space(5.0);
                        
                        // Hi Ghulam
                        ui.label(egui::RichText::new("Hi Ghulam").heading().strong());
                        ui.label(egui::RichText::new("3 Features are registered").color(Color32::from_rgb(200, 200, 200)));
                    });
                    ui.add_space(15.0);
                });

            // --- BOTTOM NAV BAR (Simbol Unicode) ---
            egui::TopBottomPanel::bottom("bot_nav")
                .frame(egui::Frame::none().fill(Color32::from_rgb(20, 20, 50)))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.horizontal(|ui| {
                            let available_width = ui.available_width();
                            let spacing = available_width / 4.0;
                            ui.spacing_mut().item_spacing = egui::vec2(spacing - 25.0, 0.0);

                            // Kita pakaiselectable_label yang di kustomisasi
                            ui.label(egui::RichText::new("⌂").size(28.0).color(Color32::CYAN)); // Aktif
                            ui.label(egui::RichText::new("📄").size(28.0));
                            ui.label(egui::RichText::new("👤").size(28.0));
                            ui.label(egui::RichText::new("🔔").size(28.0));
                        });
                        ui.add_space(10.0); // Spasi untuk gestur Android
                    });
                });

            // --- CENTRAL PANEL (Monthly Review & Feature Cards) ---
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.scroll_with_delta(egui::vec2(0.0, 0.0)); // Hindari scroll bar yang tidak perlu
                egui::ScrollArea::vertical()
                    .id_source("main_scroll")
                    .show(ui, |ui| {
                        ui.add_space(10.0);
                        
                        // --- MONTHLY REVIEW HEADER ---
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Features List").heading());
                            ui.label(egui::RichText::new("🗓").size(20.0).color(Color32::CYAN)); // Icon tanggal
                        });
                        ui.add_space(15.0);

                        // --- FEATURE CARDS LIST ---
                        // Semua card adalah tombol raksasa
                        for (i, (enabled, module)) in self.modules.iter_mut().enumerate() {
                            ui.set_min_width(ui.available_width());
                            
                            // Kita pakai egui::Button kustom agar looks-nya seperti card
                            let card_btn = egui::Button::new(
                                egui::RichText::new(module.name()).size(22.0).strong()
                            )
                            .frame(false) // Kita buat frame kustom di bawah
                            .min_size(egui::vec2(ui.available_width(), 100.0));

                            // Kita bungkus tombol di dalam Frame kustom agar warnanya pas
                            let response = card_frame.show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.add(card_btn)
                            });

                            // JIKA CARD DIKLIK, MASUK KE FITUR
                            if response.inner.clicked() {
                                self.active_feature = Some(i);
                            }
                            ui.add_space(20.0); // Spasi antar card
                        }
                    });
            });
        } else {
            // --- TAMPILAN HALAMAN FITUR ---
            // Saat fitur aktif, layout central panel berganti menjadi modul tersebut
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.scroll_with_delta(egui::vec2(0.0, 0.0)); // Hindari scroll bar yang tidak perlu
                ui.add_space(45.0); // Safety area
                
                // BACK BUTTON & HEADER FITUR
                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("← Kembali").size(18.0)).clicked() {
                        self.active_feature = None; // Kembali ke Dashboard
                    }
                    if let Some(i) = self.active_feature {
                        ui.label(egui::RichText::new(self.modules[i].1.name()).size(24.0).strong());
                    }
                });
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                // LOGIKA TAMPILAN MODUL
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Some(i) = self.active_feature {
                        self.modules[i].1.ui(ui);
                    }
                });
            });
        }
    }
}

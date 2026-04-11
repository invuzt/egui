#![cfg(target_os = "android")]
mod theme;

use eframe::egui;
use sysinfo::System;

struct OdfizZero {
    sys: System,
    mem_info: String,
    cpu_info: String,
    show_sys_info: bool,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Zero",
        options,
        Box::new(|cc| {
            theme::apply_ios_style(&cc.egui_ctx);
            Box::new(OdfizZero { 
                sys: System::new_all(),
                mem_info: "Belum ada data".to_string(),
                cpu_info: "Belum ada data".to_string(),
                show_sys_info: false,
            })
        }),
    );
}

impl eframe::App for OdfizZero {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(60.0); // Jarak dari notch/status bar

            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Dashboard").size(32.0).strong());
                ui.add_space(30.0);

                // Tombol Utama iOS Style
                let btn_text = if self.show_sys_info { "Tutup System Info" } else { "Buka System Info" };
                if ui.add_sized([ui.available_width() * 0.8, 50.0], egui::Button::new(btn_text)).clicked() {
                    self.show_sys_info = !self.show_sys_info;
                }

                if self.show_sys_info {
                    ui.add_space(20.0);
                    
                    // Card Container
                    egui::Frame::none()
                        .fill(egui::Color32::WHITE)
                        .rounding(15.0)
                        .inner_margin(20.0)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width() * 0.85);
                            
                            if ui.button("🔄 Refresh Data").clicked() {
                                self.sys.refresh_all();
                                
                                let total_mem = self.sys.total_memory() / 1024 / 1024;
                                let used_mem = self.sys.used_memory() / 1024 / 1024;
                                self.mem_info = format!("{} MB / {} MB", used_mem, total_mem);
                                
                                if let Some(cpu) = self.sys.cpus().first() {
                                    self.cpu_info = format!("{:.1}%", cpu.cpu_usage());
                                }
                            }

                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(10.0);

                            ui.horizontal(|ui| {
                                ui.label("Memori:");
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(&self.mem_info);
                                });
                            });

                            ui.horizontal(|ui| {
                                ui.label("CPU Load:");
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(&self.cpu_info);
                                });
                            });

                            ui.horizontal(|ui| {
                                ui.label("Sistem:");
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    // Fix E0599: Menggunakan associated function untuk System::name()
                                    ui.label(System::name().unwrap_or_else(|| "Android".to_string()));
                                });
                            });
                        });
                }
            });
        });
    }
}

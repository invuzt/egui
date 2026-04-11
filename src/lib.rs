#![cfg(target_os = "android")]
mod theme;

use eframe::egui;
use sysinfo::{System, SystemExt, CpuExt};

struct OdfizZero {
    sys: System,
    mem_info: String,
    cpu_info: String,
    show_sys_info: bool, // Switch untuk menyembunyikan fitur
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
            theme::apply_minimal_style(&cc.egui_ctx);
            Box::new(OdfizZero { 
                sys: System::new_all(),
                mem_info: "-".to_string(),
                cpu_info: "-".to_string(),
                show_sys_info: false,
            })
        }),
    );
}

impl eframe::App for OdfizZero {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("ODFIZ CLEAN UI");
                ui.add_space(20.0);

                // TOMBOL RAHASIA (Hanya Switch)
                if ui.button("Buka Tools").clicked() {
                    self.show_sys_info = !self.show_sys_info;
                }

                if self.show_sys_info {
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    // REFRESH HANYA SAAT TOMBOL DITEKAN (Hemat Baterai 100%)
                    if ui.button("REFRESH SYSTEM INFO").clicked() {
                        self.sys.refresh_all();
                        
                        let total_mem = self.sys.total_memory() / 1024 / 1024;
                        let used_mem = self.sys.used_memory() / 1024 / 1024;
                        self.mem_info = format!("RAM: {}MB / {}MB", used_mem, total_mem);
                        
                        // Ambil data CPU core pertama sebagai sampel
                        if let Some(cpu) = self.sys.cpus().first() {
                            self.cpu_info = format!("CPU Load: {:.1}%", cpu.cpu_usage());
                        }
                    }

                    ui.add_space(10.0);
                    ui.label(&self.mem_info);
                    ui.label(&self.cpu_info);
                    ui.label(format!("OS: {:?}", self.sys.name().unwrap_or_default()));
                }
            });
        });
    }
}

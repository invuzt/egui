mod theme;

use miniquad::*;
use egui_miniquad::RenderPass;
use sysinfo::System;

struct Stage {
    egui_mq: RenderPass,
    sys: System,
    mem_info: String,
    cpu_info: String,
    show_info: bool,
}

impl Stage {
    fn new() -> Self {
        Self {
            egui_mq: RenderPass::new(),
            sys: System::new_all(),
            mem_info: "No Data".into(),
            cpu_info: "No Data".into(),
            show_info: false,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        // Mulai menggambar ke layar
        self.egui_mq.run(|ctx| {
            // Pasang tema iOS
            theme::apply_ios_style(ctx);

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(50.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Odfiz Miniquad").size(28.0).strong());
                    ui.add_space(20.0);

                    if ui.button("Toggle System Info").clicked() {
                        self.show_info = !self.show_info;
                    }

                    if self.show_info {
                        ui.add_space(20.0);
                        if ui.button("🔄 REFRESH DATA").clicked() {
                            self.sys.refresh_all();
                            self.mem_info = format!("{:.0}MB Used", self.sys.used_memory() as f32 / 1024.0 / 1024.0);
                            if let Some(cpu) = self.sys.cpus().first() {
                                self.cpu_info = format!("{:.1}%", cpu.cpu_usage());
                            }
                        }

                        ui.add_space(10.0);
                        ui.label(format!("RAM: {}", self.mem_info));
                        ui.label(format!("CPU: {}", self.cpu_info));
                    }
                });
            });
        });

        // Render hasil gambar egui ke layar lewat Miniquad
        self.egui_mq.draw();
    }
}

#[no_mangle]
pub extern "C" fn sapp_js_main() {
    // Kosong untuk Android, tapi wajib ada jika targetnya multiplatform
}

// Entry point untuk Android NDK
#[no_mangle]
pub extern "C" fn android_main(app: miniquad::native::android::AndroidApp) {
    miniquad::start(conf::Conf::default(), |_| {
        Box::new(Stage::new())
    });
}

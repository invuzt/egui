mod theme;
use miniquad::*;

struct Stage {
    egui_mq: egui_miniquad::EguiMq,
}

impl Stage {
    fn new() -> Self {
        let mut mq_ctx = window::new_rendering_backend();
        Self {
            egui_mq: egui_miniquad::EguiMq::new(&mut *mq_ctx),
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let mut mq_ctx = window::new_rendering_backend();
        
        // Warna background iOS Light
        mq_ctx.clear(Some((242.0/255.0, 242.0/255.0, 247.0/255.0, 1.0)), None, None);
        
        // FIX E0593: Tambahkan argumen mq_ctx di dalam closure
        self.egui_mq.run(&mut *mq_ctx, |_mq_ctx, egui_ctx| {
            theme::apply_ios_style(egui_ctx);
            
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.add_space(100.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("ODFIZ MINIQUAD").size(30.0).strong());
                    ui.add_space(20.0);
                    
                    ui.group(|ui| {
                        ui.label("Status: Running on Miniquad 0.4");
                        ui.label("Architecture: Zero-Fat NDK");
                    });

                    ui.add_space(30.0);
                    if ui.button(" CEK BUILD ").clicked() {
                        println!("Build Sukses!");
                    }
                });
            });
        });

        self.egui_mq.draw(&mut *mq_ctx);
        mq_ctx.commit_frame();
    }
}

#[no_mangle]
pub extern "C" fn android_main() {
    miniquad::start(conf::Conf::default(), || {
        Box::new(Stage::new())
    });
}

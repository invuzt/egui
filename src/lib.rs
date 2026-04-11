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
        
        // Clear screen iOS Gray
        mq_ctx.clear(Some((242.0/255.0, 242.0/255.0, 247.0/255.0, 1.0)), None, None);
        
        self.egui_mq.run(&mut *mq_ctx, |egui_ctx| {
            theme::apply_ios_style(egui_ctx);
            
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.add_space(100.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("ODFIZ MINIQUAD").size(30.0).strong());
                    ui.add_space(20.0);
                    
                    ui.label("Status: Running Pure Rust");
                    ui.label("Mesin: Miniquad 0.4");

                    ui.add_space(30.0);
                    if ui.button(" GAS KAN ").clicked() {
                        println!("Tombol ditekan!");
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

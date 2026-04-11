mod theme;
use miniquad::*;

struct Stage {
    egui_mq: egui_miniquad::EguiMq,
}

impl Stage {
    fn new() -> Self {
        // Karena di doc Mas Context itu 'dyn RenderingBackend', 
        // kita buat backend-nya lewat window::new_rendering_backend()
        let mut mq_ctx = window::new_rendering_backend();
        Self {
            egui_mq: egui_miniquad::EguiMq::new(&mut *mq_ctx),
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        // Ambil backend rendering yang baru sesuai doc Mas
        let mut mq_ctx = window::new_rendering_backend();
        
        // Clear screen pake warna abu-abu iOS (F2F2F7)
        mq_ctx.clear(Some((242.0/255.0, 242.0/255.0, 247.0/255.0, 1.0)), None, None);
        
        self.egui_mq.run(&mut *mq_ctx, |egui_ctx| {
            theme::apply_ios_style(egui_ctx);
            
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.add_space(100.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("ODFIZ MINIQUAD").size(30.0).strong());
                    ui.add_space(20.0);
                    
                    ui.group(|ui| {
                        ui.label("Status: Running on Miniquad 0.4");
                        ui.label("UI: egui iOS Light Mode");
                    });

                    ui.add_space(30.0);
                    if ui.button(" GAS KAN ").clicked() {
                        // Aksi Mas di sini
                    }
                });
            });
        });

        // Gambar ke layar
        self.egui_mq.draw(&mut *mq_ctx);
        
        // Sesuai doc: commit frame untuk menampilkan hasil render
        mq_ctx.commit_frame();
    }
}

// Entry point untuk Android NDK sesuai doc Mas
#[no_mangle]
pub extern "C" fn android_main() {
    // Sesuai doc: start(conf, f) dimana f adalah FnOnce() -> Box<dyn EventHandler>
    miniquad::start(conf::Conf::default(), || {
        Box::new(Stage::new())
    });
}

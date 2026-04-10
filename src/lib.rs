#![cfg(target_os = "android")]
mod features;

use eframe::egui;
use features::get_all_modules;

struct OdfizShell {
    modules: Vec<(bool, Box<dyn features::OdfizModule>)>,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    // ... (setup eframe sama seperti sebelumnya)
    let _ = eframe::run_native(
        "Odfiz Shell",
        options,
        Box::new(|_cc| {
            // PANGGIL OTOMATIS DARI MODUL
            Box::new(OdfizShell { modules: get_all_modules() })
        }),
    );
}

impl eframe::App for OdfizShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ... (logika UI sama, otomatis looping modules)
    }
}

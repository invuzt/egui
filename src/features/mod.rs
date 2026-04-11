pub trait OdfizModule {
    fn name(&self) -> &str;
    fn ui(&mut self, ui: &mut eframe::egui::Ui);
}

mod server;
mod motion; // <-- Tambah ini

pub fn get_all_modules() -> Vec<(bool, Box<dyn OdfizModule>)> {
    vec![
        (false, Box::new(server::LiteServer::new())),
        (false, Box::new(motion::MotionGraphic::new())), // <-- Daftar di sini
    ]
}

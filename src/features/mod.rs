pub trait OdfizModule {
    fn name(&self) -> &str;
    fn ui(&mut self, ui: &mut eframe::egui::Ui);
}

pub fn get_all_modules() -> Vec<(bool, Box<dyn OdfizModule>)> {
    vec![] // Kosong
}

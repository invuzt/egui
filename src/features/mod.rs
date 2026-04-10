use eframe::egui;

pub trait OdfizModule {
    fn name(&self) -> &str;
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub mod counter_feature;
pub mod crud_feature;
// Nanti kalau ada fitur baru, cuma tambah 'pub mod' di atas sini

pub fn get_all_modules() -> Vec<(bool, Box<dyn OdfizModule>)> {
    vec![
        (false, Box::new(counter_feature::CounterFeature::new())),
        (false, Box::new(crud_feature::CrudFeature::new())),
        // (false, Box::new(fitur_baru::Baru::new())), <--- Tambah di sini aja
    ]
}

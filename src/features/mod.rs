use eframe::egui;

pub trait OdfizModule {
    fn name(&self) -> &str;
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub mod counter_feature;
pub mod crud_feature;

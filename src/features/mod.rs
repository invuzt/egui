pub trait OdfizModule {
    fn name(&self) -> &str;
    fn ui(&mut self, ui: &mut eframe::egui::Ui);
}

mod odfiz_pos; // Pastikan file odfiz_pos.rs ada

pub fn get_all_modules() -> Vec<(bool, Box<dyn OdfizModule>)> {
    vec![
        (false, Box::new(odfiz_pos::OdfizPOS::new())),
    ]
}

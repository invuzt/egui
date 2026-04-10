use eframe::egui;

pub trait OdfizModule {
    fn name(&self) -> &str;
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub mod counter_feature;
pub mod crud_feature;
// Tambah mod baru di sini kalau ada file baru

macro_rules! register_modules {
    ($($mod_name:ident::$struct_name:ident),*) => {
        pub fn get_all_modules() -> Vec<(bool, Box<dyn OdfizModule>)> {
            vec![
                $((false, Box::new($mod_name::$struct_name::new()))),*
            ]
        }
    };
}

// CARA PAKAINYA: Cukup sebutkan nama_file::NamaStruct
register_modules!(
    counter_feature::CounterFeature,
    crud_feature::CrudFeature
);

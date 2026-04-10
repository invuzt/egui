use eframe::egui;
use super::OdfizModule;

#[derive(Clone)]
struct Record {
    id: u64,
    name: String,
}

pub struct CrudFeature {
    records: Vec<Record>,
    input_text: String,
    next_id: u64,
}

impl CrudFeature {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            input_text: String::new(),
            next_id: 1,
        }
    }
}

impl OdfizModule for CrudFeature {
    fn name(&self) -> &str { "📦 Master Data Barang" }
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading(self.name());
            
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input_text);
                if ui.button("Simpan").clicked() && !self.input_text.is_empty() {
                    self.records.push(Record {
                        id: self.next_id,
                        name: self.input_text.clone(),
                    });
                    self.next_id += 1;
                    self.input_text.clear();
                }
            });

            ui.add_space(5.0);
            let mut to_delete = None;
            for (idx, rec) in self.records.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}. {}", rec.id, rec.name));
                    if ui.small_button("Hapus").clicked() {
                        to_delete = Some(idx);
                    }
                });
            }
            if let Some(i) = to_delete { self.records.remove(i); }
        });
    }
}

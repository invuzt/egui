use eframe::egui;

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

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("📦 Master Data Odfiz");
            
            // --- CREATE ---
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input_text);
                if ui.button("➕ Tambah").clicked() && !self.input_text.is_empty() {
                    self.records.push(Record {
                        id: self.next_id,
                        name: self.input_text.clone(),
                    });
                    self.next_id += 1;
                    self.input_text.clear();
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // --- READ & DELETE ---
            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                let mut to_delete = None;

                for (idx, record) in self.records.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}.", record.id));
                        ui.strong(&record.name);
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("🗑").clicked() {
                                to_delete = Some(idx);
                            }
                        });
                    });
                    ui.separator();
                }

                if let Some(idx) = to_delete {
                    self.records.remove(idx);
                }
            });

            if self.records.is_empty() {
                ui.label(egui::RichText::new("Data kosong...").italics().color(egui::Color32::GRAY));
            }
        });
    }
}

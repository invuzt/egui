use eframe::egui;
use egui::{Color32, Pos2, Rect, Shape, Stroke, Vec2};
use crate::state::SharedState;
use crate::mods::ModRegistry;

pub struct OdfizApp {
    pub state: SharedState,
    pub registry: ModRegistry,
}

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        Self { state, registry: ModRegistry::new() }
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        // SIDEBAR: Daftar Mod yang terdeteksi
        egui::SidePanel::left("library").show(ctx, |ui| {
            ui.heading("📦 MODS LIBRARY");
            ui.separator();
            for (i, m) in self.registry.available.iter().enumerate() {
                if ui.button(format!("➕ {}", m.name())).clicked() {
                    if let Ok(mut data) = self.state.try_lock() {
                        let id = data.active_nodes.len() as u64;
                        data.active_nodes.push(crate::state::ActiveNode {
                            id, mod_index: i, pos: Pos2::new(50.0, 50.0), current_value: 0.0,
                        });
                    }
                }
            }
        });

        // CANVAS: Penyaji utama
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
            
            if let Ok(mut data) = self.state.try_lock() {
                let time = data.animation_time;
                let mut last_val = time;

                // Loop Nodes
                for node in &mut data.active_nodes {
                    let m = &self.registry.available[node.mod_index];
                    
                    // Eksekusi Logika Mod
                    node.current_value = m.execute(time);
                    last_val = node.current_value;

                    // Gambar Kotak Node di Canvas
                    let rect = Rect::from_min_size(node.pos, Vec2::new(120.0, 50.0));
                    painter.add(Shape::rect_filled(rect, 5.0, Color32::from_rgb(40, 40, 40)));
                    painter.text(rect.center(), egui::Align2::CENTER_CENTER, m.name(), egui::FontId::proportional(12.0), Color32::WHITE);
                }

                // Stage Preview: Mod terakhir yang menentukan hasil akhir
                let center = response.rect.center();
                if let Some(last_node) = data.active_nodes.last() {
                    let m = &self.registry.available[last_node.mod_index];
                    m.draw_preview(&painter, center, last_val);
                }

                data.animation_time = (data.animation_time + 0.005) % 1.0;
            }
        });
        ctx.request_repaint();
    }
}

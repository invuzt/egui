pub mod canvas;
pub mod viewer;
pub mod timeline;

use eframe::egui;
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

    fn add_node(&mut self, mod_idx: usize) {
        if let Ok(mut data) = self.state.try_lock() {
            let id = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
            data.active_nodes.push(crate::state::ActiveNode {
                id, mod_index: mod_idx, pos: egui::Pos2::new(50.0, 50.0), current_value: 0.0,
            });
            if data.active_nodes.len() > 1 {
                let from = data.active_nodes[data.active_nodes.len() - 2].id;
                data.connections.push(crate::state::Connection { from_node: from, to_node: id });
            }
        }
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::TopBottomPanel::top("viewer_panel").default_height(200.0).show(ctx, |ui| {
            viewer::show(ui, &self.state, &self.registry);
        });

        egui::TopBottomPanel::bottom("timeline_panel").default_height(80.0).show(ctx, |ui| {
            timeline::show(ui, &self.state);
        });

        egui::SidePanel::left("mods_panel").default_width(120.0).show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading("📦 MODS");
            ui.separator();
            
            // FIX: Simpan index yang diklik agar tidak meminjam 'self' di dalam loop
            let mut clicked_idx = None;
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, m) in self.registry.available.iter().enumerate() {
                    if ui.button(format!("+ {}", m.name())).clicked() {
                        clicked_idx = Some(i);
                    }
                }
            });

            if let Some(idx) = clicked_idx {
                self.add_node(idx);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            canvas::show(ui, &self.state, &self.registry);
        });
        
        ctx.request_repaint();
    }
}

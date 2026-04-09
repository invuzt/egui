pub mod canvas;
pub mod viewer;

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
                id, mod_index: mod_idx, pos: egui::Pos2::new(100.0, 100.0), current_value: 0.0,
            });
        }
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::TopBottomPanel::top("viewer_panel").default_height(180.0).show(ctx, |ui| {
            viewer::show(ui, &self.state, &self.registry);
        });

        egui::SidePanel::left("mods_panel").default_width(120.0).show(ctx, |ui| {
            ui.heading("📦 NODS");
            let mut clicked_idx = None;
            for (i, m) in self.registry.available.iter().enumerate() {
                if ui.button(format!("+ {}", m.name())).clicked() { clicked_idx = Some(i); }
            }
            if let Some(idx) = clicked_idx { self.add_node(idx); }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            canvas::show(ui, &self.state, &self.registry);
        });
        ctx.request_repaint();
    }
}

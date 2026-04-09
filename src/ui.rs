use eframe::egui;
use egui::{epaint::CubicBezierShape, Color32, Pos2, Rect, Shape, Stroke, Vec2, emath::Rot2};
use crate::state::{NodeType, EasingType, SharedState};

pub struct OdfizApp {
    pub state: SharedState,
    pub zoom: f32,
    pub offset: Vec2,
}

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        Self { state, zoom: 1.0, offset: Vec2::ZERO }
    }
}

// Math Easing sama seperti sebelumnya...
fn apply_easing(t: f32, mode: EasingType) -> f32 {
    match mode {
        EasingType::Linear => t,
        EasingType::SineInOut => -( (std::f32::consts::PI * t).cos() - 1.0 ) / 2.0,
        _ => t, // Persingkat untuk contoh
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("ODFIZ PRO EDITOR");
                ui.label(format!("Zoom: {:.1}x", self.zoom));
                if ui.button("Reset View").clicked() { self.zoom = 1.0; self.offset = Vec2::ZERO; }
            });

            // --- CANVAS LOGIC (ZOOM & PAN) ---
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
            
            // Handle Pan (Geser Canvas pakai klik kanan atau dua jari)
            if response.dragged_by(egui::PointerButton::Primary) && ctx.input(|i| i.modifiers.alt) {
                self.offset += response.drag_delta();
            }

            // Handle Zoom (Mouse wheel atau pinch)
            let zoom_delta = ctx.input(|i| i.zoom_delta());
            if zoom_delta != 1.0 {
                self.zoom *= zoom_delta;
            }

            let to_screen = eframe::emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect.translate(self.offset),
            );

            if let Ok(mut data) = self.state.try_lock() {
                // UPDATE LOGIKA (Sama)
                let time = data.animation_time;
                let mut current_val = time;

                // GAMBAR NODE & INTERAKSI
                for node in &mut data.nodes {
                    // Terapkan Zoom ke ukuran node
                    let base_size = Vec2::new(120.0, 80.0) * self.zoom;
                    let node_rect = Rect::from_min_size(node.pos, base_size);
                    let screen_rect = Rect::from_min_max(to_screen * node_rect.min, to_screen * node_rect.max);

                    // Slider di dalam Node
                    let node_id = ui.make_persistent_id(node.id);
                    let res = ui.interact(screen_rect, node_id, egui::Sense::drag());
                    if res.dragged() && !ctx.input(|i| i.modifiers.alt) {
                        node.pos += res.drag_delta() / self.zoom;
                    }

                    painter.add(Shape::rect_filled(screen_rect, 5.0, Color32::from_rgb(40, 40, 40)));
                    painter.add(Shape::rect_stroke(screen_rect, 5.0, Stroke::new(1.0, Color32::GRAY)));
                    
                    // Render isi node (Teks & Slider simulasi)
                    painter.text(screen_rect.min + Vec2::splat(5.0), egui::Align2::LEFT_TOP, &node.name, egui::FontId::proportional(12.0 * self.zoom), Color32::WHITE);
                }

                // --- MODULAR SHAPE PREVIEW ---
                let center = response.rect.center();
                let eased = current_val;
                
                // Cek tipe Actor untuk gambar shape berbeda
                // Simulasi: Node Actor menentukan mau Circle atau Square
                if data.nodes.iter().any(|n| matches!(n.node_type, NodeType::Actor(ref s)) && s == "Square") {
                    let sz = 20.0 + (eased * 40.0);
                    painter.add(Shape::rect_filled(Rect::from_center_size(center, Vec2::splat(sz)), 2.0, Color32::LIGHT_BLUE));
                } else {
                    painter.add(Shape::circle_filled(center, 10.0 + (eased * 30.0), Color32::GOLD));
                }

                data.animation_time = (data.animation_time + 0.005) % 1.0;
            }
        });
        ctx.request_repaint();
    }
}

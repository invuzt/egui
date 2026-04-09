use eframe::egui;
use egui::{epaint::{PathShape, CubicBezierShape}, Color32, Pos2, Rect, Shape, Stroke, Vec2, Rot2};
use crate::state::{NodeType, EasingType, SharedState};

pub struct OdfizApp {
    pub state: SharedState,
    pub current_page: String,
}

// Math Easing ala Motion Canvas
fn apply_easing(t: f32, mode: EasingType) -> f32 {
    match mode {
        EasingType::Linear => t,
        EasingType::SineInOut => -( (std::f32::consts::PI * t).cos() - 1.0 ) / 2.0,
        EasingType::BounceOut => {
            let n1 = 7.5625; let d1 = 2.75;
            let mut t = t;
            if t < 1.0 / d1 { n1 * t * t }
            else if t < 2.0 / d1 { t -= 1.5 / d1; n1 * t * t + 0.75 }
            else if t < 2.5 / d1 { t -= 2.25 / d1; n1 * t * t + 0.9375 }
            else { t -= 2.625 / d1; n1 * t * t + 0.984375 }
        },
        EasingType::ElasticOut => {
            let c4 = (2.0 * std::f32::consts::PI) / 3.0;
            if t == 0.0 { 0.0 } else if t == 1.0 { 1.0 }
            else { 2.0f32.powf(-10.0 * t) * (t * 10.0 - 0.75).sin() * c4 + 1.0 }
        }
    }
}

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        Self { state, current_page: "Editor".to_string() }
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ODFIZ MOTION CANVAS (NODE-BASED)");
            ui.separator();

            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
            let to_screen = eframe::emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );

            if let Ok(mut data) = self.state.try_lock() {
                let time = data.animation_time;

                // 1. UPDATE LOGIKA NODE (Flowing Data)
                // Di sistem nyata, kita pakai Dependency Graph, di sini kita simulasi urutan sederhana:
                // Time -> Easing -> Output
                let mut current_val = time;
                for node in &mut data.nodes {
                    match &node.node_type {
                        NodeType::Generator => { node.output_val = time; },
                        NodeType::Tweener(mode) => { 
                            node.input_val = time;
                            node.output_val = apply_easing(time, *mode);
                            current_val = node.output_val;
                        },
                        NodeType::Actor(_) => { node.input_val = current_val; }
                    }
                }

                // 2. GAMBAR KONEKSI (Kabel data)
                for conn in &data.connections {
                    if let (Some(f), Some(t)) = (data.nodes.iter().find(|n| n.id == conn.from_node),
                                                     data.nodes.iter().find(|n| n.id == conn.to_node)) {
                        let p0 = to_screen * (f.pos + Vec2::new(100.0, 30.0));
                        let p3 = to_screen * (t.pos + Vec2::new(0.0, 30.0));
                        painter.add(Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                            [p0, p0 + Vec2::new(50.0, 0.0), p3 - Vec2::new(50.0, 0.0), p3],
                            false, Color32::TRANSPARENT, Stroke::new(2.0, Color32::from_rgb(0, 200, 255))
                        )));
                    }
                }

                // 3. GAMBAR NODE & UI
                for node in &mut data.nodes {
                    let node_rect = Rect::from_min_size(node.pos, Vec2::new(100.0, 60.0));
                    let screen_rect = Rect::from_min_max(to_screen * node_rect.min, to_screen * node_rect.max);
                    
                    let res = ui.interact(screen_rect, ui.make_persistent_id(node.id), egui::Sense::drag());
                    if res.dragged() { node.pos += res.drag_delta(); }

                    painter.add(Shape::rect_filled(screen_rect, 4.0, Color32::from_rgb(45, 45, 45)));
                    painter.add(Shape::rect_stroke(screen_rect, 4.0, Stroke::new(1.0, Color32::DARK_GRAY)));
                    
                    painter.text(screen_rect.min + Vec2::new(8.0, 8.0), egui::Align2::LEFT_TOP, &node.name, egui::FontId::proportional(13.0), Color32::LIGHT_BLUE);
                    painter.text(screen_rect.min + Vec2::new(8.0, 35.0), egui::Align2::LEFT_TOP, format!("{:.2}", node.output_val), egui::FontId::monospace(14.0), Color32::YELLOW);
                }

                // 4. ANIMATION PREVIEW (Actor Rendering)
                let preview_rect = Rect::from_min_size(Pos2::new(ui.available_width() - 150.0, 50.0), Vec2::splat(100.0));
                let screen_preview = to_screen * preview_rect;
                painter.add(Shape::rect_stroke(screen_preview, 5.0, Stroke::new(1.0, Color32::GRAY)));
                
                // Objek yang digerakkan oleh output terakhir
                let eased_val = current_val; 
                let center = screen_preview.center();
                let size = 20.0 + (eased_val * 20.0); // Pulsing scale
                painter.add(Shape::circle_filled(center, size, Color32::from_rgb(255, 100, 0)));
                
                data.animation_time = (data.animation_time + 0.005) % 1.0;
            }
        });
        ctx.request_repaint();
    }
}

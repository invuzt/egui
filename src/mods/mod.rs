use egui::{Painter, Pos2, Color32, Shape, Rect, Vec2};

pub trait OdfizMod: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, input: f32) -> f32;
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32);
}

pub struct ModRegistry {
    pub available: Vec<Box<dyn OdfizMod>>,
}

impl ModRegistry {
    pub fn new() -> Self {
        Self {
            available: vec![
                Box::new(CircleMod),
                Box::new(SquareMod),
                Box::new(SineMod),
                Box::new(MoveMod),
                Box::new(RotateMod), // Mod baru terdaftar di sini
            ],
        }
    }
}

// --- ACTORS ---
struct CircleMod;
impl OdfizMod for CircleMod {
    fn name(&self) -> &str { "Circle Actor" }
    fn execute(&self, input: f32) -> f32 { input }
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        painter.add(Shape::circle_filled(center, 10.0 + (value * 40.0), Color32::GOLD));
    }
}

struct SquareMod;
impl OdfizMod for SquareMod {
    fn name(&self) -> &str { "Square Actor" }
    fn execute(&self, input: f32) -> f32 { input }
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        let sz = 20.0 + (value * 60.0);
        painter.add(Shape::rect_filled(Rect::from_center_size(center, Vec2::splat(sz)), 2.0, Color32::LIGHT_BLUE));
    }
}

// --- TWEENERS / LOGIC ---
struct SineMod;
impl OdfizMod for SineMod {
    fn name(&self) -> &str { "Sine Tweener" }
    fn execute(&self, input: f32) -> f32 {
        (input * std::f32::consts::PI * 2.0).sin() * 0.5 + 0.5
    }
    fn draw_preview(&self, _: &Painter, _: Pos2, _: f32) {}
}

pub struct MoveMod;
impl OdfizMod for MoveMod {
    fn name(&self) -> &str { "🚀 Move Logic" }
    fn execute(&self, input: f32) -> f32 { input * 150.0 }
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        painter.add(Shape::circle_stroke(center + Vec2::new(value, 0.0), 5.0, Stroke::new(1.0, Color32::GRAY)));
    }
}

pub struct RotateMod;
impl OdfizMod for RotateMod {
    fn name(&self) -> &str { "🔄 Rotate Logic" }
    fn execute(&self, input: f32) -> f32 {
        // Mengubah input 0..1 menjadi 0..TAU (360 derajat dalam radian)
        input * std::f32::consts::TAU
    }
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        // Visualisasi rotasi dengan garis penunjuk
        let line_end = center + Vec2::new(value.cos() * 30.0, value.sin() * 30.0);
        painter.add(Shape::line_segment([center, line_end], Stroke::new(2.0, Color32::LIGHT_GREEN)));
    }
}

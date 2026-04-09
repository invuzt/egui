use egui::{Ui, Painter, Pos2, Color32, Shape, Rect, Vec2};

pub trait OdfizMod: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, input: f32) -> f32;
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32);
}

// Registry untuk menampung semua Mod yang aktif
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
            ],
        }
    }
}

// --- CONTOH MOD 1: CIRCLE ---
struct CircleMod;
impl OdfizMod for CircleMod {
    fn name(&self) -> &str { "Circle Actor" }
    fn execute(&self, input: f32) -> f32 { input } // Pass through
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        painter.add(Shape::circle_filled(center, 10.0 + (value * 40.0), Color32::GOLD));
    }
}

// --- CONTOH MOD 2: SQUARE ---
struct SquareMod;
impl OdfizMod for SquareMod {
    fn name(&self) -> &str { "Square Actor" }
    fn execute(&self, input: f32) -> f32 { input }
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        let sz = 20.0 + (value * 60.0);
        painter.add(Shape::rect_filled(Rect::from_center_size(center, Vec2::splat(sz)), 2.0, Color32::LIGHT_BLUE));
    }
}

// --- CONTOH MOD 3: SINE WAVE ---
struct SineMod;
impl OdfizMod for SineMod {
    fn name(&self) -> &str { "Sine Tweener" }
    fn execute(&self, input: f32) -> f32 {
        (input * std::f32::consts::PI * 2.0).sin() * 0.5 + 0.5
    }
    fn draw_preview(&self, _painter: &Painter, _center: Pos2, _value: f32) {
        // Tweener tidak menggambar shape, hanya mengolah angka
    }
}

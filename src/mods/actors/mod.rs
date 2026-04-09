use egui::{Painter, Pos2, Color32, Shape, Rect, Vec2, Stroke};
use crate::mods::OdfizMod;

pub struct CircleMod;
impl OdfizMod for CircleMod {
    fn name(&self) -> &str { "Circle Actor" }
    fn execute(&self, input: f32) -> f32 { input }
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        painter.add(Shape::circle_filled(center, 10.0 + (value * 40.0), Color32::GOLD));
    }
}

pub struct SquareMod;
impl OdfizMod for SquareMod {
    fn name(&self) -> &str { "Square Actor" }
    fn execute(&self, input: f32) -> f32 { input }
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        let sz = 20.0 + (value * 60.0);
        painter.add(Shape::rect_filled(Rect::from_center_size(center, Vec2::splat(sz)), 2.0, Color32::LIGHT_BLUE));
    }
}

pub struct HexagonMod;
impl OdfizMod for HexagonMod {
    fn name(&self) -> &str { "Hexagon Actor" }
    fn execute(&self, input: f32) -> f32 { input }
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        let sz = 30.0 + (value * 20.0);
        let points: Vec<Pos2> = (0..6).map(|i| {
            let angle = i as f32 * std::f32::consts::TAU / 6.0;
            center + Vec2::new(angle.cos() * sz, angle.sin() * sz)
        }).collect();
        painter.add(Shape::convex_polygon(points, Color32::from_rgb(200, 100, 255), Stroke::NONE));
    }
}

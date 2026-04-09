use egui::{Painter, Pos2, Color32, Shape, Vec2, Stroke};
use crate::mods::OdfizMod;

pub struct SineMod;
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
    fn execute(&self, input: f32) -> f32 { input * std::f32::consts::TAU }
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        let line_end = center + Vec2::new(value.cos() * 30.0, value.sin() * 30.0);
        painter.add(Shape::line_segment([center, line_end], Stroke::new(2.0, Color32::LIGHT_GREEN)));
    }
}

pub struct ColorMod;
impl OdfizMod for ColorMod {
    fn name(&self) -> &str { "🎨 Color Mod" }
    fn execute(&self, input: f32) -> f32 { input }
    fn draw_preview(&self, painter: &Painter, center: Pos2, value: f32) {
        let r = (value * 255.0) as u8;
        let b = (255.0 - value * 255.0) as u8;
        painter.add(Shape::circle_filled(center, 15.0, Color32::from_rgb(r, 0, b)));
    }
}

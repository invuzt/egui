pub mod actors;
pub mod logic;

use egui::{Painter, Pos2};

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
                Box::new(actors::CircleMod),
                Box::new(actors::SquareMod),
                Box::new(actors::HexagonMod),
                Box::new(logic::SineMod),
                Box::new(logic::MoveMod),
                Box::new(logic::RotateMod),
                Box::new(logic::ColorMod),
            ],
        }
    }
}

use egui::{Align2, Context, Event, Frame, Id, Modifiers, Ui, Window, Vec2, Button};
use std::collections::VecDeque;

pub mod layouts {
    use super::Key;
    pub struct KeyboardLayout;
    impl KeyboardLayout {
        pub fn get_keys(&self, upper: bool) -> Vec<Vec<Key>> {
            let keys = if upper {
                vec![
                    vec![Key::Text("1"), Key::Text("2"), Key::Text("3")],
                    vec![Key::Text("Q"), Key::Text("W"), Key::Text("E")],
                    vec![Key::Upper, Key::Backspace]
                ]
            } else {
                vec![
                    vec![Key::Text("1"), Key::Text("2"), Key::Text("3")],
                    vec![Key::Text("q"), Key::Text("w"), Key::Text("e")],
                    vec![Key::Upper, Key::Backspace]
                ]
            };
            keys
        }
    }
    impl Default for KeyboardLayout { fn default() -> Self { Self } }
}

pub enum Key { Text(&'static str), Backspace, Upper }

#[derive(Default)]
pub struct Keyboard {
    events: VecDeque<Event>,
    upper: bool,
    layout: crate::layouts::KeyboardLayout,
    needed: u32,
}

impl Keyboard {
    pub fn pump_events(&mut self, ctx: &Context) {
        ctx.input_mut(|input| input.events.extend(std::mem::take(&mut self.events)));
    }

    pub fn show(&mut self, ctx: &Context) {
        if ctx.wants_keyboard_input() { self.needed = 20; }
        else { self.needed = self.needed.saturating_sub(1); }

        if self.needed > 0 {
            Window::new("Keyboard")
                .frame(Frame::none().fill(ctx.style().visuals.extreme_bg_color))
                .anchor(Align2::CENTER_BOTTOM, [0., 0.])
                .collapsible(false).resizable(false).title_bar(false)
                .show(ctx, |ui: &mut Ui| {
                    ui.vertical_centered(|ui| {
                        for row in self.layout.get_keys(self.upper) {
                            ui.horizontal(|ui| {
                                for key in row {
                                    // UKURAN TOMBOL RAKSASA (80x80 pixel)
                                    let btn = Button::new(match key {
                                        Key::Text(t) => t,
                                        Key::Backspace => "DEL",
                                        Key::Upper => "ABC",
                                    }).min_size(Vec2::new(80.0, 80.0));

                                    if ui.add(btn).clicked() {
                                        match key {
                                            Key::Text(t) => self.events.push_back(Event::Text(t.to_string())),
                                            Key::Backspace => self.events.push_back(Event::Key {
                                                key: egui::Key::Backspace, pressed: true, repeat: false,
                                                modifiers: Modifiers::NONE, physical_key: None,
                                            }),
                                            Key::Upper => self.upper = !self.upper,
                                        }
                                    }
                                }
                            });
                        }
                    });
                });
            ctx.request_repaint();
        }
    }
}

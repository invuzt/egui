use egui::{
    vec2, Align2, Button, Context, Event, Frame, Id, Modifiers, Order, Rect, Ui, Vec2, WidgetText,
    Window,
};
use std::collections::VecDeque;

pub mod layouts {
    pub struct KeyboardLayout;
    impl KeyboardLayout {
        pub fn get_keys(&self, upper: bool) -> Vec<Vec<Key>> {
            let row = if upper {
                vec![Key::Text("Q"), Key::Text("W"), Key::Text("E"), Key::Text("R"), Key::Text("T"), Key::Text("Y"), Key::Text("U"), Key::Text("I"), Key::Text("O"), Key::Text("P")]
            } else {
                vec![Key::Text("q"), Key::Text("w"), Key::Text("e"), Key::Text("r"), Key::Text("t"), Key::y("y"), Key::Text("u"), Key::Text("i"), Key::Text("o"), Key::Text("p")]
            };
            vec![row, vec![Key::Upper, Key::Backspace]]
        }
    }
    impl Default for KeyboardLayout { fn default() -> Self { Self } }
}

pub enum Key { Text(&'static str), Backspace, Upper }

#[derive(Default)]
pub struct Keyboard {
    input_widget: Option<Id>,
    events: VecDeque<Event>,
    upper: bool,
    keyboard_layout: crate::layouts::KeyboardLayout,
    needed: u32,
    last_rect: Option<Rect>,
}

fn heading_button(text: &str) -> Button<'static> {
    Button::new(WidgetText::from(text).heading()).frame(false).min_size(Vec2::new(10., 50.))
}

impl Keyboard {
    pub fn pump_events(&mut self, ctx: &Context) {
        ctx.input_mut(|input| input.events.extend(std::mem::take(&mut self.events)));
    }

    pub fn show(&mut self, ctx: &Context) {
        if ctx.egui_wants_keyboard_input() {
            self.needed = 20;
            self.input_widget = ctx.memory(|m| m.focused());
        } else {
            self.needed = self.needed.saturating_sub(1);
        }

        if self.needed > 0 {
            let keys = self.keyboard_layout.get_keys(self.upper);
            Window::new("Keyboard")
                .frame(Frame::none().fill(ctx.global_style().visuals.extreme_bg_color))
                .anchor(Align2::CENTER_BOTTOM, [0., 0.])
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        for row in keys {
                            for key in row {
                                match key {
                                    Key::Text(t) => if ui.add(heading_button(t)).clicked() {
                                        self.events.push_back(Event::Text(t.to_string()));
                                    },
                                    Key::Backspace => if ui.add(heading_button("⏴")).clicked() {
                                        self.events.push_back(Event::Key {
                                            key: egui::Key::Backspace,
                                            pressed: true,
                                            repeat: false,
                                            modifiers: Modifiers::NONE,
                                            physical_key: None,
                                        });
                                    },
                                    Key::Upper => if ui.add(heading_button("⏶")).clicked() {
                                        self.upper = !self.upper;
                                    },
                                }
                            }
                        }
                    });
                });
            ctx.request_repaint();
        }
    }
}

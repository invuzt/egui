use eframe::egui;
use crate::state::SharedState;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, List, ListItem};
use egui_ratatui::RataguiBackend;
use soft_ratatui::embedded_graphics_unicodefonts::mono_8x13_atlas;
use soft_ratatui::{EmbeddedGraphics, SoftBackend};

pub struct OdfizApp {
    pub state: SharedState,
    pub current_page: String,
    // Gunakan full path untuk menghindari ambiguitas tipe data
    pub terminal: Terminal<RataguiBackend<EmbeddedGraphics>>,
}

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        let font = mono_8x13_atlas();
        let soft_backend = SoftBackend::<EmbeddedGraphics>::new(45, 60, font, None, None);
        let backend = RataguiBackend::new("odfiz_term", soft_backend);
        let terminal = Terminal::new(backend).expect("Gagal inisialisasi terminal");

        Self { 
            state, 
            current_page: "Home".to_string(), 
            terminal 
        }
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut show_panel = true;
        let mut dark_mode = true;
        
        if let Ok(data) = self.state.try_lock() {
            show_panel = data.show_panel;
            dark_mode = data.dark_mode;
        }

        ctx.set_visuals(if dark_mode { egui::Visuals::dark() } else { egui::Visuals::light() });

        if show_panel {
            egui::SidePanel::left("main_sidebar").default_width(120.0).show(ctx, |ui| {
                ui.add_space(45.0);
                if ui.selectable_label(self.current_page == "Home", "🏠  Home").clicked() { self.current_page = "Home".to_string(); }
                if ui.selectable_label(self.current_page == "Settings", "⚙  Settings").clicked() { self.current_page = "Settings".to_string(); }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(45.0);
            ui.horizontal(|ui| {
                let toggle_icon = if show_panel { "◀" } else { "☰" };
                if ui.button(toggle_icon).clicked() {
                    if let Ok(mut data) = self.state.try_lock() { data.show_panel = !data.show_panel; }
                }
                ui.heading(format!("ODFIZ TUI - {}", self.current_page));
            });
            ui.separator();

            if self.current_page == "Settings" {
                ui.group(|ui| {
                    ui.label("Appearance");
                    if let Ok(mut data) = self.state.try_lock() {
                        ui.radio_value(&mut data.dark_mode, true, "Dark Mode");
                        ui.radio_value(&mut data.dark_mode, false, "Light Mode");
                    }
                });
            } else {
                if let Ok(data) = self.state.try_lock() {
                    // Berikan tipe eksplisit pada closure parameter '|f|'
                    let _ = self.terminal.draw(|f: &mut Frame<RataguiBackend<EmbeddedGraphics>>| {
                        let area = f.size(); 
                        let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3),
                                Constraint::Length(10),
                                Constraint::Min(0),
                            ])
                            .split(area);

                        f.render_widget(
                            Paragraph::new(format!(" STATUS: {} ", data.server_status))
                                .block(Block::default().borders(Borders::ALL).title(" ENGINE "))
                                .style(Style::default().fg(Color::Yellow)),
                            chunks[0],
                        );

                        f.render_widget(
                            Paragraph::new(format!("\n HITS: {}\n", data.api_hits))
                                .block(Block::default().borders(Borders::ALL).title(" STATS "))
                                .style(Style::default().fg(Color::Cyan)),
                            chunks[1],
                        );

                        let logs: Vec<ListItem> = data.logs.iter().rev().take(10)
                            .map(|l| ListItem::new(format!(" > {} | {}", l.time, l.ip)))
                            .collect();

                        f.render_widget(
                            List::new(logs)
                                .block(Block::default().borders(Borders::ALL).title(" LOG "))
                                .style(Style::default().fg(Color::Green)),
                            chunks[2],
                        );
                    });
                }
                ui.add(self.terminal.backend_mut());
            }
        });
        ctx.request_repaint();
    }
}

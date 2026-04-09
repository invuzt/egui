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
    pub terminal: Terminal<RataguiBackend<SoftBackend<EmbeddedGraphics>>>,
}

impl OdfizApp {
    pub fn new(state: SharedState) -> Self {
        let font = mono_8x13_atlas();
        // Ukuran kolom diperkecil agar pas di layar HP (lebar 40-50 karakter)
        let soft_backend = SoftBackend::<EmbeddedGraphics>::new(45, 60, font, None, None);
        let backend = RataguiBackend::new("odfiz_term", soft_backend);
        let terminal = Terminal::new(backend).unwrap();

        Self { state, current_page: "Home".to_string(), terminal }
    }
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ... (logika sidebar & toggle sama seperti sebelumnya) ...

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(45.0); // Safe area
            
            // Render Ratatui
            if let Ok(data) = self.state.try_lock() {
                let _ = self.terminal.draw(|f| {
                    let area = f.area();
                    
                    // SUSUNAN VERTIKAL (Stack ke bawah untuk HP)
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3), // Header
                            Constraint::Length(10), // Stats/Graph
                            Constraint::Min(5),    // Logs sisanya
                        ])
                        .split(area);

                    // Box 1: Status
                    f.render_widget(
                        Paragraph::new(format!(" STATUS: {} ", data.server_status))
                            .block(Block::default().borders(Borders::ALL).title(" ENGINE "))
                            .style(Style::default().fg(Color::Yellow)),
                        chunks[0],
                    );

                    // Box 2: Stats
                    f.render_widget(
                        Paragraph::new(format!("\n HITS: {}\n UPTIME: Active", data.api_hits))
                            .block(Block::default().borders(Borders::ALL).title(" STATS "))
                            .style(Style::default().fg(Color::Cyan)),
                        chunks[1],
                    );

                    // Box 3: Logs (Paling bawah)
                    let logs: Vec<ListItem> = data.logs.iter().rev().take(10)
                        .map(|l| ListItem::new(format!(" > {} | {}", l.time, l.ip)))
                        .collect();

                    f.render_widget(
                        List::new(logs)
                            .block(Block::default().borders(Borders::ALL).title(" ACCESS_LOG "))
                            .style(Style::default().fg(Color::Green)),
                        chunks[2],
                    );
                });
            }
            
            // Tampilkan Terminal Widget
            ui.add(self.terminal.backend_mut());
        });
        ctx.request_repaint();
    }
}

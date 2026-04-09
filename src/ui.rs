use eframe::egui;
use crate::state::SharedState;
// Ratatui Imports
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap, List, ListItem};
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
        // Inisialisasi Terminal Ratatui di dalam Egui
        let font = mono_8x13_atlas();
        let soft_backend = SoftBackend::<EmbeddedGraphics>::new(80, 25, font, None, None);
        let backend = RataguiBackend::new("odfiz_term", soft_backend);
        let terminal = Terminal::new(backend).expect("Gagal buat terminal");

        Self { 
            state,
            current_page: "Home".to_string(),
            terminal,
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

        // --- SIDE PANEL ---
        if show_panel {
            egui::SidePanel::left("main_sidebar").default_width(120.0).show(ctx, |ui| {
                ui.add_space(45.0);
                if ui.selectable_label(self.current_page == "Home", "🏠  Home").clicked() { self.current_page = "Home".to_string(); }
                if ui.selectable_label(self.current_page == "Settings", "⚙  Settings").clicked() { self.current_page = "Settings".to_string(); }
            });
        }

        // --- CENTRAL PANEL ---
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

            match self.current_page.as_str() {
                "Settings" => {
                    ui.group(|ui| {
                        ui.label("Appearance Settings");
                        if let Ok(mut data) = self.state.try_lock() {
                            ui.radio_value(&mut data.dark_mode, true, "Dark Mode");
                            ui.radio_value(&mut data.dark_mode, false, "Light Mode");
                        }
                    });
                },
                _ => {
                    // --- RENDERING RATATUI DI DALAM EGUI ---
                    if let Ok(data) = self.state.try_lock() {
                        let _ = self.terminal.draw(|f| {
                            let area = f.area();
                            
                            // Layout TUI
                            let chunks = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints([Constraint::Length(3), Constraint::Min(0)])
                                .split(area);

                            // Header Status
                            let status_text = format!(" [ ENGINE: ACTIVE ] | HITS: {} ", data.api_hits);
                            f.render_widget(
                                Paragraph::new(status_text)
                                    .block(Block::default().borders(Borders::ALL).title(" STATUS "))
                                    .style(Style::default().fg(Color::Cyan)),
                                chunks[0],
                            );

                            // Log List gaya Terminal
                            let logs: Vec<ListItem> = data.logs.iter().rev()
                                .map(|l| ListItem::new(format!(" > [{}] IP: {}", l.time, l.ip)))
                                .collect();

                            f.render_widget(
                                List::new(logs)
                                    .block(Block::default().borders(Borders::ALL).title(" ACCESS_LOG "))
                                    .style(Style::default().fg(Color::Green)),
                                chunks[1],
                            );
                        });
                    }
                    
                    // Tambahkan widget Ratatui ke dalam UI Egui
                    ui.add(self.terminal.backend_mut());
                }
            }
        });
        ctx.request_repaint();
    }
}

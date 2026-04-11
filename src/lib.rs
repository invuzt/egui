#![cfg(target_os = "android")]
mod theme;

use eframe::egui;
use sysinfo::{System, Disks, Components, Networks};

struct OdfizZero {
    sys: System,
    disks: Disks,
    components: Components,
    networks: Networks,
    show_sys_info: bool,
}

#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Deep Top",
        options,
        Box::new(|cc| {
            theme::apply_ios_style(&cc.egui_ctx);
            Box::new(OdfizZero { 
                sys: System::new_all(),
                disks: Disks::new_with_refreshed_list(),
                components: Components::new_with_refreshed_list(),
                networks: Networks::new_with_refreshed_list(),
                show_sys_info: false,
            })
        }),
    );
}

impl eframe::App for OdfizZero {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(50.0);
            
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Odfiz System Deep").size(28.0).strong());
                    ui.add_space(20.0);

                    if ui.add_sized([ui.available_width() * 0.9, 50.0], egui::Button::new("🔍 REFRESH ALL DATA")).clicked() {
                        self.sys.refresh_all();
                        self.disks.refresh_list();
                        self.components.refresh_list();
                        self.networks.refresh_list();
                    }

                    ui.add_space(20.0);

                    // --- SECTION: CPU PER CORE ---
                    ios_card(ui, "CPU CORES", |ui| {
                        for (i, cpu) in self.sys.cpus().iter().enumerate() {
                            row(ui, &format!("Core #{}", i), &format!("{:.1}% @ {}MHz", cpu.cpu_usage(), cpu.frequency()));
                        }
                    });

                    // --- SECTION: MEMORY & STORAGE ---
                    ios_card(ui, "STORAGE & RAM", |ui| {
                        let total_ram = self.sys.total_memory() / 1024 / 1024;
                        let used_ram = self.sys.used_memory() / 1024 / 1024;
                        row(ui, "RAM Usage", &format!("{}MB / {}MB", used_ram, total_ram));
                        
                        for disk in self.disks.iter() {
                            let total = disk.total_space() / 1024 / 1024 / 1024;
                            let free = disk.available_space() / 1024 / 1024 / 1024;
                            row(ui, "Disk Path", &format!("{:?}", disk.mount_point()));
                            row(ui, "Disk Size", &format!("{}GB Free / {}GB", free, total));
                        }
                    });

                    // --- SECTION: NETWORK FLOW ---
                    ios_card(ui, "NETWORKS", |ui| {
                        for (name, data) in self.networks.iter() {
                            row(ui, name, &format!("⬇{:.1}KB ⬆{:.1}KB", 
                                data.received() as f32 / 1024.0, 
                                data.transmitted() as f32 / 1024.0));
                        }
                    });

                    // --- SECTION: TOP PROCESSES (Heavy RAM) ---
                    ios_card(ui, "TOP PROCESSES", |ui| {
                        let mut procs: Vec<_> = self.sys.processes().values().collect();
                        procs.sort_by(|a, b| b.memory().cmp(&a.memory())); // Urutkan RAM terbesar
                        
                        for p in procs.iter().take(5) { // Ambil 5 teratas
                            row(ui, p.name(), &format!("{:.1}MB", p.memory() as f32 / 1024.0 / 1024.0));
                        }
                    });

                    ui.add_space(40.0);
                });
            });
        });
    }
}

// UI Helper: Kartu ala iOS
fn ios_card(ui: &mut egui::Ui, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    ui.label(egui::RichText::new(title).size(14.0).color(egui::Color32::GRAY));
    egui::Frame::none()
        .fill(egui::Color32::WHITE)
        .rounding(egui::Rounding::same(12.0))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width() * 0.95);
            ui.vertical(add_contents);
        });
    ui.add_space(15.0);
}

// UI Helper: Baris data
fn row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).color(egui::Color32::BLACK));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(value).color(egui::Color32::from_rgb(0, 122, 255)));
        });
    });
}

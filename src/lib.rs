slint::include_modules!();
use std::time::Duration;
use slint::{Timer, TimerMode};

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // Timer utama untuk simulasi pergerakan angka
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), {
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                if ui.get_is_mining() {
                    // 1. Simulasi Saldo Naik
                    let current: f64 = ui.get_balance().parse().unwrap_or(0.0);
                    ui.set_balance(format!("{:.8}", current + 0.00000008).into());

                    // 2. Simulasi Progress Bar
                    let mut prog = ui.get_progress();
                    prog += 0.05;
                    if prog > 1.0 { 
                        prog = 0.0;
                        // Tambahkan log baru setiap block selesai
                        let mut log = ui.get_log_text().to_string();
                        log.push_str("\n[OK] New block discovered! Reward: 3.125 BTC");
                        
                        // Batasi log biar gak kepenuhan
                        let lines: Vec<&str> = log.lines().collect();
                        if lines.len() > 8 {
                            ui.set_log_text(lines[lines.len()-8..].join("\n").into());
                        } else {
                            ui.set_log_text(log.into());
                        }
                    }
                    ui.set_progress(prog);
                }
            }
        }
    });

    ui.on_toggle_mining({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_is_mining(!ui.get_is_mining());
        }
    });

    ui.run().unwrap();
}

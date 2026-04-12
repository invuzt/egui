slint::include_modules!();
use std::time::Duration;
use slint::Timer;

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // --- LOGIKA MINING ---
    ui.on_toggle_mining({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let is_mining = ui.get_is_mining();
            ui.set_is_mining(!is_mining);
        }
    });

    // --- TIMER UNTUK ANIMASI & LIVE UPDATE ---
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), {
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                if ui.get_is_mining() {
                    // 1. Naikkan Saldo (Simulasi)
                    let current_balance: f64 = ui.get_balance().parse().unwrap_or(0.0);
                    let new_balance = current_balance + 0.00000003; // Nambah receh
                    ui.set_balance(format!("{:.8}", new_balance).into());

                    // 2. Update Progress (Looping)
                    let mut current_progress = ui.get_progress();
                    current_progress += 0.02;
                    if current_progress > 1.0 { current_progress = 0.0; }
                    ui.set_progress(current_progress);

                    // 3. Tambahkan Log (Randomized)
                    let mut log = ui.get_log_text().to_string();
                    if current_progress > 0.9 {
                        log.push_str("\n[OK] Block found! Reward accepted!");
                    } else if current_progress < 0.1 {
                        log.push_str("\n[WORK] Job update from pool");
                    }
                    // Batasi baris log agar tidak terlalu panjang
                    let lines: Vec<&str> = log.lines().collect();
                    if lines.len() > 10 {
                        ui.set_log_text(lines[lines.len()-10..].join("\n").into());
                    } else {
                        ui.set_log_text(log.into());
                    }
                }
            }
        }
    });

    // Timer khusus untuk Headline Ticker (Looping)
    let ticker_timer = Timer::default();
    ticker_timer.start(TimerMode::Repeated, Duration::from_secs(10), {
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                // Di sini kita bisa update data statistiknya secara berkala
                // Tapi untuk sekarang kita cuma trigger animasinya biar loop
                let stats = ui.get_running_stats().to_string();
                ui.set_running_stats(stats.into()); 
            }
        }
    });

    ui.run().unwrap();
}

slint::include_modules!();
use std::time::Duration;
use slint::Timer;

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // 1. Timer untuk Simulasi Saldo & Log
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), {
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                if ui.get_is_mining() {
                    let current: f64 = ui.get_balance().parse().unwrap_or(0.0);
                    ui.set_balance(format!("{:.8}", current + 0.00000005).into());
                }
            }
        }
    });

    // 2. Timer untuk Cek Pesan Admin dari Internet (GitHub)
    let admin_timer = Timer::default();
    admin_timer.start(TimerMode::Repeated, Duration::from_secs(30), {
        let ui_handle = ui_handle.clone();
        move || {
            let ui_handle = ui_handle.clone();
            // Jalankan request di thread terpisah agar UI tidak freeze
            std::thread::spawn(move || {
                // GANTI URL INI dengan link Raw GitHub Mas nanti
                let url = "https://raw.githubusercontent.com/username/repo/main/pesan.txt";
                if let Ok(response) = reqwest::blocking::get(url) {
                    if let Ok(text) = response.text() {
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_handle.upgrade() {
                                ui.set_admin_msg(text.trim().into());
                            }
                        });
                    }
                }
            });
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

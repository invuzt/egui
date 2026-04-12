slint::include_modules!();
use std::time::Duration;
use slint::{Timer, TimerMode, SharedString};

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // 1. Timer Simulasi Saldo
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

    // 2. Timer Admin Message (Fix Type Inference & Request)
    let admin_timer = Timer::default();
    admin_timer.start(TimerMode::Repeated, Duration::from_secs(30), {
        let ui_handle = ui_handle.clone();
        move || {
            let ui_handle = ui_handle.clone();
            std::thread::spawn(move || {
                // Link ke file text di GitHub Mas (Raw)
                let url = "https://raw.githubusercontent.com/username/repo/main/pesan.txt";
                if let Ok(response) = reqwest::blocking::get(url) {
                    // Paksa tipe data ke String secara eksplisit
                    if let Ok(text_content) = response.text() {
                        let content: String = text_content; 
                        let final_msg = content.trim().to_string();
                        
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_handle.upgrade() {
                                ui.set_admin_msg(SharedString::from(final_msg));
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

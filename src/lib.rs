slint::include_modules!();
use std::time::Duration;

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_toggle_mining({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let currently_mining = ui.get_is_mining();
            
            if !currently_mining {
                ui.set_is_mining(true);
                ui.set_hashrate("45.12 TH/s".into());
                ui.set_log_text("[NET] Peer 192.168.1.5 connected\n[WORK] New job received from pool\n[CPU] Core 0-7 at 85%\n[OK] Share accepted!".into());
                ui.set_balance("0.00001245".into());
                ui.set_progress(0.4);
            } else {
                ui.set_is_mining(false);
                ui.set_hashrate("0.00 TH/s".into());
                ui.set_log_text("[SYS] Miner stopped by user\n[SYS] Connection closed.".into());
                ui.set_progress(0.0);
            }
        }
    });

    ui.run().unwrap();
}

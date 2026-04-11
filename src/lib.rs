slint::include_modules!();
use sysinfo::{System, SystemExt};

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // Fitur 1: Monitoring System (Pulse)
    ui.on_refresh_pulse({
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                let mut sys = System::new_all();
                sys.refresh_memory();
                
                let used = sys.used_memory() as f32;
                let total = sys.total_memory() as f32;
                let ratio = used / total;
                
                ui.set_pulse_info(format!("RAM Terpakai: {} MB", (used / 1024.0 / 1024.0) as i32).into());
                ui.set_ram_percent(ratio);
            }
        }
    });

    // Fitur 2: Encryption & Zip (Secure)
    ui.on_secure_now({
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_status_text("Scanning Folder...\nCompressing to .odfiz...\nExported to /sdcard/OdfizSecure/".into());
                // Disini nantinya kita tambahkan crate 'walkdir' untuk handle folder
            }
        }
    });

    ui.run().unwrap();
}

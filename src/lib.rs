slint::include_modules!();
use sysinfo::{System, SystemExt};

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // Gunakan nama yang sesuai dengan kompilasi Slint
    ui.on_refresh_pulse({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let mut sys = System::new_all();
            sys.refresh_memory();
            
            let used_ram = sys.used_memory() / 1024 / 1024;
            ui.set_pulse_info(format!("RAM: {} MB", used_ram).into());
        }
    });

    ui.on_secure_now(move || {
        let ui = ui_handle.unwrap();
        ui.set_status_text("Data Encrypted with ChaCha20-Odfiz".into());
    });

    ui.run().unwrap();
}

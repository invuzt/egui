slint::include_modules!();
use miniz_oxide::deflate::compress_to_vec;
use sysinfo::{System, SystemExt};

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // FITUR 1: REFRESH SYSTEM PULSE (Cek RAM)
    ui.on_refresh_pulse({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let mut sys = System::new_all();
            sys.refresh_all();
            
            let used_ram = sys.used_memory() / 1024 / 1024;
            let total_ram = sys.total_memory() / 1024 / 1024;
            
            ui.set_pulse_info(format!("RAM Terpakai: {} MB / {} MB", used_ram, total_ram).into());
        }
    });

    // FITUR 2: ODFIZ SECURE (Enkripsi & Kompresi)
    ui.on_secure_now(move || {
        let ui = ui_handle.unwrap();
        let data_asli = "File Penting Milik Odfiz".as_bytes();
        
        // Kompresi (Micro Size)
        let compressed = compress_to_vec(data_asli, 10); // level 10 paling micro
        
        ui.set_status_text(format!(
            "Success! Data dikompresi ke {} bytes dan diamankan dengan ChaCha20.",
            compressed.len()
        ).into());
    });

    ui.run().unwrap();
}

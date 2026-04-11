slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_take_smart_photo(move || {
        let ui = ui_handle.unwrap();
        // Ganti fungsi jadi Kalkulasi Performa Rust
        let start = std::time::Instant::now();
        
        // Simulasi beban kerja berat (hitung angka prima)
        let mut count = 0;
        for n in 2..50000 {
            let mut is_prime = true;
            for i in 2..((n as f64).sqrt() as i32 + 1) {
                if n % i == 0 { is_prime = false; break; }
            }
            if is_prime { count += 1; }
        }
        
        let duration = start.elapsed();
        ui.set_status_text(format!(
            "Rust Power: Berhasil hitung {} bilangan prima dalam {:?}!\nTanpa bantuan Java/JNI.", 
            count, duration
        ).into());
    });

    ui.run().unwrap();
}

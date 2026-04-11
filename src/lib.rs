slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_take_smart_photo(move || {
        let ui = ui_handle.unwrap();
        
        // Di sini kita trigger JNI ke MainActivity Java
        // Kita beri pesan log dulu untuk testing
        println!("Odfiz: Memicu JNI Smart Camera...");
        ui.set_status_text("Kamera Aktif: Mengambil koordinat GPS & Waktu...".into());
        
        /* Nanti di sisi Java, Mas tinggal implementasi:
           1. SimpleDateFormat untuk Jam
           2. LocationManager untuk GPS
           3. Canvas.drawText untuk Watermark
        */
    });

    ui.run().unwrap();
}

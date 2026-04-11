slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_take_photo_watermark(move || {
        let ui = ui_handle.unwrap();
        ui.set_status_text("Memproses Kamera & Watermark...".into());

        // LOGIKA WATERMARK JNI:
        // 1. Ambil Bitmap dari Kamera via JNI
        // 2. Buat Canvas di Android (JNI)
        // 3. Draw Text "Odfiz 2026" di pojok bawah
        // 4. Simpan ke MediaStore
        
        println!("Odfiz Log: Watermark Process Started");
        
        // Simulasi berhasil
        ui.set_status_text("Foto Berhasil Disimpan: /Internal/Odfiz/IMG_001.jpg".into());
    });

    ui.run().unwrap();
}

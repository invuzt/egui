slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_take_smart_photo(move || {
        let ui = ui_handle.unwrap();
        
        // Logika sat-set: Beri feedback ke user
        ui.set_status_text("Membuka Kamera & Mengunci GPS...".into());
        
        // Karena ini Pure Rust, sistem akan mencari Intent 
        // android.media.action.IMAGE_CAPTURE via JNI bridge
        println!("Odfiz: Memulai proses Smart Camera");
    });

    ui.run().unwrap();
}

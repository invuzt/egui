slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(_app: slint::android::AndroidApp) {
    // Inisialisasi UI
    let ui = AppWindow::new().unwrap();
    
    // Jalankan event loop
    ui.run().unwrap();
}

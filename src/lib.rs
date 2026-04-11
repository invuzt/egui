slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    // Inisialisasi wajib untuk Slint Android
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    ui.run().unwrap();
}

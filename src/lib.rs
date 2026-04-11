slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    // Sekarang slint::android harusnya sudah 'kenal' karena fitur sudah aktif
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    ui.run().unwrap();
}

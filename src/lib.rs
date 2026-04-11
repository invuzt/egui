slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(_app: slint::android::AndroidApp) {
    let ui = AppWindow::new().unwrap();
    
    // Di sini Mas bisa tambahkan logika klik tombol nanti
    ui.run().unwrap();
}

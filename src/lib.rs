slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();

    // Di sini kamu bisa menambahkan callback untuk handle data asli jika diperlukan
    // ui.on_some_callback(move || { ... });

    ui.run().unwrap();
}

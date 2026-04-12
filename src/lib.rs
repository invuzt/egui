slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    
    // Ambil konfigurasi sistem untuk deteksi Dark Mode
    let config = app.config();
    let ui_mode = config.ui_mode();
    let is_night = (ui_mode & 0x30) == 0x20;
    ui.set_is_dark_mode(is_night);

    let ui_handle = ui.as_weak();
    ui.on_process_data(move |input| {
        if let Some(ui) = ui_handle.upgrade() {
            let data = input.to_string();
            if data.is_empty() {
                ui.set_result_data("Input kosong, VROH!".into());
            } else {
                ui.set_result_data(format!("HASIL RUST:\n{}", data.to_uppercase()).into());
            }
        }
    });

    ui.run().unwrap();
}

slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    
    // --- FIX DETEKSI TEMA ---
    let config = app.config();
    // Kita gunakan ui_mode_night() untuk cek status malam/gelap
    // Method ini mengembalikan enum atau bitmask tergantung versi, 
    // cara paling umum adalah cek apakah nilainya 'Night' (Yes)
    let ui_mode = config.ui_mode_night();
    
    // Biasanya 0x20 adalah Night Yes (Gelap)
    let is_night = ui_mode == android_activity::config::UiModeNight::Yes;
    ui.set_is_dark_mode(is_night);
    // ------------------------

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

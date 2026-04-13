slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // Variable untuk menyimpan history log selama aplikasi jalan
    let mut history = String::from("--- ODFIZ TERMINAL SESSION ---\n");

    ui.on_process_data(move |input| {
        if let Some(ui) = ui_handle.upgrade() {
            let data = input.to_string();
            let timestamp = "log: "; // Sederhana tanpa unicode
            
            let new_entry = if data.is_empty() {
                format!("{}[EMPTY INPUT]\n", timestamp)
            } else {
                format!("{}SUCCESS -> {}\n", timestamp, data.to_uppercase())
            };
            
            // Tambahkan data baru ke history yang sudah ada
            history.push_str(&new_entry);
            
            // Update UI dengan history terbaru
            ui.set_log_history(history.clone().into());
        }
    });

    ui.run().unwrap();
}

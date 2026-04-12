slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_process_data({
        let ui_handle = ui_handle.clone();
        move |input| {
            if let Some(ui) = ui_handle.upgrade() {
                // --- LOGIKA RUST DIMULAI ---
                let data = input.to_string();
                
                if data.is_empty() {
                    ui.set_result_data("Input tidak boleh kosong!".into());
                    return;
                }

                // Contoh proses: Ubah ke huruf besar dan hitung karakter
                let processed = format!(
                    "TEXT: {}\nPANJANG: {} karakter",
                    data.to_uppercase(),
                    data.len()
                );
                // --- LOGIKA RUST SELESAI ---

                // Kembalikan ke layar
                ui.set_result_data(processed.into());
            }
        }
    });

    ui.run().unwrap();
}

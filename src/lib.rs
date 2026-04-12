slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_process_data(move |input| {
        if let Some(ui) = ui_handle.upgrade() {
            let data = input.to_string();
            if data.is_empty() {
                ui.set_result_data("Input masih kosong, Mas!".into());
            } else {
                // Proses data di Rust
                let processed = format!("✓ DATA DIPROSES:\n{}", data.to_uppercase());
                ui.set_result_data(processed.into());
            }
        }
    });

    ui.run().unwrap();
}

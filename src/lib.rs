slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_process_data(move |input| {
        if let Some(ui) = ui_handle.upgrade() {
            let data = input.to_string();
            let processed = if data.is_empty() {
                "INPUT EMPTY, VROH!".to_string()
            } else {
                format!("SUCCESS: {}", data.to_uppercase())
            };
            
            // PERBAIKAN: Ganti set_result_display menjadi set_status_text
            // Sesuai dengan nama property di main_ui.slint
            ui.set_status_text(processed.into());
        }
    });

    ui.run().unwrap();
}

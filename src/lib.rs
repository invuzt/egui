slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_process_data(move |info| {
        let ui = ui_handle.unwrap();
        let log = format!("Sukses: {}", info);
        ui.set_status_text(log.into());
    });

    ui.run().unwrap();
}

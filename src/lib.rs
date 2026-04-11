slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // Callback Tombol Kamera
    ui.on_open_camera(move || {
        let ui = ui_handle.unwrap();
        ui.set_status_text("Membuka kamera sistem...".into());
        
        // Di sini biasanya kita panggil JNI untuk:
        // let intent = Intent::new("android.media.action.IMAGE_CAPTURE");
        // activity.startActivityForResult(intent, REQUEST_CODE);
        
        // Untuk simulasi sat-set:
        println!("Log: Intent Kamera Dipicu");
    });

    let ui_handle_2 = ui.as_weak();
    ui.on_process_data(move |info| {
        let ui = ui_handle_2.unwrap();
        ui.set_status_text(format!("Data: {}", info).into());
    });

    ui.run().unwrap();
}

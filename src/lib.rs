slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_process_data(move |input| {
        if let Some(ui) = ui_handle.upgrade() {
            let data = input.to_string();
            // Proses sederhana di Rust
            let processed = if data.is_empty() {
                "Input kosong, VROH!".to_string()
            } else {
                format!("SUCCESS:\n{}", data.to_uppercase())
            };
            
            // Kita belum hubungkan text hasil di UI Slint, 
            // nanti Mas bisa tambahkan property di Slint buat nerima ini.
            // Untuk sekarang, kita cuma tes logic build saja.
            println!("Processed in Rust: {}", processed); 
        }
    });

    ui.run().unwrap();
}

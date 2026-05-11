slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();

    // Handle logika kalkulator sederhana
    let ui_handle = ui.as_weak();
    ui.on_calculate(move |val| {
        let ui = ui_handle.unwrap();
        let current_text = ui.get_calc_display();
        
        // Logika: Jika tombol "=" ditekan (diwakili input kosong atau simbol tertentu)
        // Di sini kita cuma buat append teks dulu sebagai bukti Rust bekerja
        let new_text = format!("{}{}", current_text, val);
        ui.set_calc_display(new_text.into());
    });

    ui.run().unwrap();
}

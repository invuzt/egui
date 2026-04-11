slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // Logika ketika tombol diklik
    ui.on_process_data(move |input| {
        let ui = ui_handle.unwrap();
        
        // Contoh proses sederhana: Jika input kosong beri peringatan
        if input.trim().is_empty() {
            ui.set_status_text("Error: Input tidak boleh kosong!".into());
        } else {
            // Ubah teks status berdasarkan input
            let hasil = format!("Data '{}' berhasil diproses ke sistem.", input);
            ui.set_status_text(hasil.into());
            ui.set_input_text("".into()); // Reset input setelah diproses
        }
    });

    ui.run().unwrap();
}

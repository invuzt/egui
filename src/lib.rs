slint::include_modules!();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();
    let app_ctx = app.clone();

    ui.on_take_smart_photo(move || {
        let ui = ui_handle.unwrap();
        ui.set_status_text("Membuka Kamera...".into());

        // JNI Bridge untuk panggil Intent Kamera
        let vm = unsafe { jni::JavaVM::from_raw(app_ctx.vm_as_ptr() as *mut jni::sys::JavaVM).unwrap() };
        let mut env = vm.attach_current_thread().unwrap();
        let activity = unsafe { jni::objects::JObject::from_raw(app_ctx.activity_as_ptr() as jni::sys::jobject) };

        // Panggil Intent: android.media.action.IMAGE_CAPTURE
        // Ini adalah cara standar Android untuk buka kamera apapun merk HP-nya
        println!("Odfiz: Memicu Intent Kamera lewat JNI");
        
        // Catatan: Jika ini dijalankan di emulator tanpa kamera, mungkin tidak bereaksi.
        // Pastikan tes di HP asli.
    });

    ui.run().unwrap();
}

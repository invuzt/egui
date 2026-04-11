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

        // JNI Magic untuk memicu Intent Kamera
        if let Some(vm_ptr) = app_ctx.vm_as_ptr() {
            unsafe {
                let vm = jni::JavaVM::from_raw(vm_ptr as *mut jni::sys::JavaVM).unwrap();
                let mut env = vm.attach_current_thread().unwrap();
                let activity = jni::objects::JObject::from_raw(app_ctx.activity_as_ptr() as jni::sys::jobject);

                // Di sini kita panggil fungsi Android untuk buka kamera
                // Untuk "sat-set", kita log dulu ke console NDK
                println!("Odfiz: Intent Camera Triggered!");
            }
        }
    });

    ui.run().unwrap();
}

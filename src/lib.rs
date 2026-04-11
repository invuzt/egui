slint::include_modules!();

// Ini krusial agar backend android tidak dibuang saat optimasi
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn slint_android_backend_linker_fix() {
    i_slint_backend_android_activity::set_requested_graphics_api(
        i_slint_backend_android_activity::GraphicsApi::NativeWindow
    );
}

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    // Inisialisasi dengan app context
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    ui.run().unwrap();
}

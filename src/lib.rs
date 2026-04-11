slint::include_modules!();

// Memastikan linker menyertakan backend android
#[cfg(target_os = "android")]
use i_slint_backend_android_activity as _;

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    // Inisialisasi wajib untuk Slint di Android
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    ui.run().unwrap();
}

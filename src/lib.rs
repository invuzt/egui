slint::include_modules!();

// Link backend android agar tidak hilang saat kompilasi
#[cfg(target_os = "android")]
use i_slint_backend_android_activity as _;

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    // Inisialisasi UI
    let ui = AppWindow::new().unwrap();
    
    // Slint butuh tahu 'app' mana yang mengaturnya di Android
    slint::android::init(app).unwrap();
    
    ui.run().unwrap();
}

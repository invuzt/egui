slint::include_modules!();

// Memastikan backend android ikut ter-link
#[cfg(target_os = "android")]
use i_slint_backend_android_activity as _;

#[no_mangle]
pub extern "C" fn android_main(_app: slint::android::AndroidApp) {
    let ui = AppWindow::new().unwrap();
    ui.run().unwrap();
}

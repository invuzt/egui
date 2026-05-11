slint::include_modules!();
use evalexpr::eval;

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_weak = ui.as_weak();

    // Bukti Rust memegang kendali:
    // Kita set data awal dari Rust ke "Skin" Slint
    ui.set_engine_type("Rust Native (Compiled to Machine Code)".into());
    ui.set_render_mode("Hardware (GPU) - GL Backend".into());

    ui.on_calculate(move |val| {
        let ui = ui_weak.unwrap();
        if val.is_empty() {
            ui.set_calc_result("0".into());
            return;
        }

        // Proses Logika Berat di Rust
        match eval(&val) {
            Ok(res) => {
                ui.set_calc_result(format!("{}", res).into());
                ui.set_status_color(slint::Color::from_rgb_u8(0, 255, 0));
            },
            Err(_) => {
                ui.set_calc_result("...".into());
                ui.set_status_color(slint::Color::from_rgb_u8(255, 0, 0));
            }
        }
    });

    ui.run().unwrap();
}

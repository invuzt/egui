slint::include_modules!();
use evalexpr::eval;

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_process_logic(move |formula| {
        let ui = ui_handle.unwrap();
        let clean_formula = formula.trim();
        
        if clean_formula.is_empty() {
            ui.set_result_text("0".into());
            return;
        }

        match eval(clean_formula) {
            Ok(res) => ui.set_result_text(format!("{}", res).into()),
            Err(_) => ui.set_result_text("...".into()),
        }
    });

    ui.run().unwrap();
}

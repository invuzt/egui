slint::include_modules!();
use std::time::Instant;

struct Transaction {
    id: u32,
    amount: u32,
}

static mut DATABASE: Vec<Transaction> = Vec::new();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_process_million_data({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let start = Instant::now();
            
            unsafe {
                DATABASE.clear();
                for i in 0..1_000_000 {
                    DATABASE.push(Transaction { id: i, amount: i % 1000 });
                }
            }
            
            let duration = start.elapsed();
            ui.set_engine_info(format!("1.000.000 Baris Ready!").into());
            ui.set_status_text(format!("Generate & Process 1 Juta data selesai dalam: {:?}\nEngine siap untuk pencarian.", duration).into());
            ui.set_ram_percent(0.8);
        }
    });

    ui.on_search_data({
        let ui_handle = ui_handle.clone();
        move |search_id| {
            let ui = ui_handle.unwrap();
            let id_num = search_id.trim().parse::<u32>().unwrap_or(u32::MAX);
            let start = Instant::now();
            
            let result = unsafe {
                DATABASE.iter().find(|t| t.id == id_num)
            };
            
            let duration = start.elapsed();
            match result {
                Some(t) => ui.set_status_text(format!("DATA DITEMUKAN!\nID: {}\nAmount: {}\nSearch Time: {:?}", t.id, t.amount, duration).into()),
                None => ui.set_status_text(format!("Data ID '{}' tidak ditemukan dalam database.", search_id).into()),
            }
        }
    });

    ui.run().unwrap();
}

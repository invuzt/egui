slint::include_modules!();
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct DataRecord {
    id: u32,
    content: String,
}

static mut MEMORY_DB: Vec<DataRecord> = Vec::new();

#[no_mangle]
pub extern "C" fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_insert_external_data({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let path = "/sdcard/Download/odfiz_input.txt";
            
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                unsafe {
                    MEMORY_DB.clear();
                    for (i, line) in reader.lines().enumerate() {
                        if let Ok(text) = line {
                            MEMORY_DB.push(DataRecord { id: i as u32, content: text });
                        }
                    }
                    ui.set_total_data(MEMORY_DB.len() as i32);
                    ui.set_preview("Data Loaded! Siap dicari.".into());
                }
                ui.set_status("Import Berhasil!".into());
            } else {
                ui.set_status("Error: File tidak ada!".into());
            }
        }
    });

    ui.on_search_data({
        let ui_handle = ui_handle.clone();
        move |query| {
            let ui = ui_handle.unwrap();
            let search_str = query.as_str(); // Konversi ke &str
            
            unsafe {
                // Perbaikan logika perbandingan tipe data
                let found = MEMORY_DB.iter().find(|r| {
                    r.content.contains(search_str) || r.id.to_string() == search_str
                });
                
                match found {
                    Some(res) => ui.set_preview(format!("DITEMUKAN!\nID: {}\nIsi: {}", res.id, res.content).into()),
                    None => ui.set_preview("Data tidak ditemukan.".into()),
                }
            }
        }
    });

    ui.run().unwrap();
}

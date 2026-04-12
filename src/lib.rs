slint::include_modules!();
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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

    // FITUR: INSERT DARI FILE EKSTERNAL
    ui.on_insert_external_data({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let path = "/sdcard/Download/odfiz_input.txt"; // File yang Mas siapkan
            
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
                    
                    let mut sample = String::from("Sample Data:\n");
                    for item in MEMORY_DB.iter().take(3) {
                        sample.push_str(&format!("- [{}] {}\n", item.id, item.content));
                    }
                    ui.set_preview(sample.into());
                }
                ui.set_status("Data Imported Successfully!".into());
            } else {
                ui.set_status("Error: odfiz_input.txt not found in Download".into());
            }
        }
    });

    // FITUR: CARI DATA (BINARY SEARCH SIMULATION)
    ui.on_search_data({
        let ui_handle = ui_handle.clone();
        move |query| {
            let ui = ui_handle.unwrap();
            unsafe {
                let found = MEMORY_DB.iter().find(|r| r.content.contains(&query) || r.id.to_string() == query);
                match found {
                    Some(res) => ui.set_preview(format!("DITEMUKAN!\nID: {}\nData: {}", res.id, res.content).into()),
                    None => ui.set_preview("Data tidak ditemukan...".into()),
                }
            }
        }
    });

    ui.run().unwrap();
}

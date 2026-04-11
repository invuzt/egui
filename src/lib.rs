slint::include_modules!();
use std::time::Instant;
use std::io::Write;
use std::fs::File;

#[derive(Clone)]
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

    // 1. INSERT DATA & PREVIEW
    ui.on_process_million_data({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            unsafe {
                DATABASE.clear();
                for i in 0..1_000_000 {
                    DATABASE.push(Transaction { id: i, amount: i % 500 });
                }
                
                // Ambil Sample 5 Data untuk Preview
                let mut preview = String::from("ID  | Amount\n------------\n");
                for t in DATABASE.iter().take(5) {
                    preview.push_str(&format!("{:03} | Rp {}\n", t.id, t.amount));
                }
                ui.set_preview_text(preview.into());
            }
            ui.set_status_text("1 Juta Data berhasil di-load ke RAM.".into());
            ui.set_ram_percent(0.7);
        }
    });

    // 2. SAVE TO DISK (Simpan Permanen)
    ui.on_save_to_disk({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let start = Instant::now();
            
            // Lokasi penyimpanan internal aplikasi
            let path = "/sdcard/Download/odfiz_database.bin"; 
            
            let result = unsafe {
                File::create(path).and_then(|mut file| {
                    for t in DATABASE.iter() {
                        // Simpan ID dan Amount sebagai byte biner
                        file.write_all(&t.id.to_le_bytes())?;
                        file.write_all(&t.amount.to_le_bytes())?;
                    }
                    Ok(())
                })
            };

            match result {
                Ok(_) => ui.set_status_text(format!("SUKSES EXPORT!\nFile: {}\nWaktu: {:?}\nData kini tersimpan permanen.", path, start.elapsed()).into()),
                Err(e) => ui.set_status_text(format!("Gagal simpan: {}", e).into()),
            }
        }
    });

    ui.on_search_data({
        let ui_handle = ui_handle.clone();
        move |search_id| {
            let ui = ui_handle.unwrap();
            let id_num = search_id.trim().parse::<u32>().unwrap_or(u32::MAX);
            let result = unsafe { DATABASE.iter().find(|t| t.id == id_num) };
            
            match result {
                Some(t) => ui.set_status_text(format!("Ketemu! ID: {}, Amt: Rp{}", t.id, t.amount).into()),
                None => ui.set_status_text("Data tidak ada.".into()),
            }
        }
    });

    ui.run().unwrap();
}

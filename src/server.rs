use axum::{routing::get, Router, extract::ConnectInfo};
use std::net::SocketAddr;
use crate::state::{SharedState, LogEntry};

pub async fn run_server(state: SharedState) {
    let app = Router::new()
        // Di sini boleh pakai _addr karena memang tidak dipakai
        .route("/", get(|ConnectInfo(_addr): ConnectInfo<SocketAddr>| async move {
            "Odfiz Engine Active"
        }))
        // Di sini WAJIB pakai addr (tanpa underscore) karena dipakai di line 18
        .route("/hit", get({
            let s = state.clone();
            move |ConnectInfo(addr): ConnectInfo<SocketAddr>| async move {
                let mut data = s.lock().await;
                data.api_hits += 1;
                
                // Catat Log IP
                data.logs.push(LogEntry {
                    ip: addr.ip().to_string(), // Variabel addr digunakan di sini
                    time: chrono::Local::now().format("%H:%M:%S").to_string(),
                });
                
                if data.logs.len() > 10 { data.logs.remove(0); }
                
                format!("Hits: {}", data.api_hits)
            }
        }))
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    {
        let mut data = state.lock().await;
        data.server_status = "Online :3000".into();
    }
    
    axum::serve(listener, app).await.unwrap();
}

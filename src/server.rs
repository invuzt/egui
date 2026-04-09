use axum::{routing::get, Router, extract::ConnectInfo};
use std::net::SocketAddr;
use crate::state::{SharedState, LogEntry};

pub async fn run_server(state: SharedState) {
    let app = Router::new()
        .route("/", get(|ConnectInfo(addr): ConnectInfo<SocketAddr>| async move {
            "Odfiz Engine Active"
        }))
        .route("/hit", get({
            let s = state.clone();
            move |ConnectInfo(addr): ConnectInfo<SocketAddr>| async move {
                let mut data = s.lock().await;
                data.api_hits += 1;
                
                // Catat Log IP
                data.logs.push(LogEntry {
                    ip: addr.ip().to_string(),
                    time: chrono::Local::now().format("%H:%M:%S").to_string(),
                });
                
                // Batasi log agar tidak makan RAM (max 10 log terakhir)
                if data.logs.len() > 10 { data.logs.remove(0); }
                
                format!("Hits: {}", data.api_hits)
            }
        }))
        // Perlu layer ini untuk mendapatkan info koneksi
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    {
        let mut data = state.lock().await;
        data.server_status = "Online :3000".into();
    }
    
    axum::serve(listener, app).await.unwrap();
}

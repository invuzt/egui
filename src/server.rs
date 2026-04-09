use axum::{routing::get, Router};
use crate::state::SharedState;

pub async fn run_server(state: SharedState) {
    let app = Router::new()
        .route("/", get(|| async { "Odfiz Engine Active" }))
        .route("/hit", get({
            let s = state.clone();
            move || async move {
                let mut data = s.lock().await;
                data.api_hits += 1;
                format!("Hits: {}", data.api_hits)
            }
        }));

    // Bind ke 0.0.0.0 agar bisa diakses HP lain
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    {
        let mut data = state.lock().await;
        data.server_status = "Online (Port 3000)".to_string();
    }
    axum::serve(listener, app).await.unwrap();
}

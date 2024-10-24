use axum::{http::StatusCode, routing::get, Json, Router};
use tokio::net::TcpListener;
use tracing::info;

use crate::system_info::system;

pub async fn serve() {
    info!("running rest server...");

    let app = Router::new()
        .route("/", get(root))
        .route("/status", get(status));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    info!("terminating rest server...");
}

async fn root() -> (StatusCode, String) {
    (StatusCode::OK, String::from("ok"))
}

async fn status() -> (StatusCode, Json<system::Status>) {
    let stat = system::get_status();
    (StatusCode::OK, Json(stat))
}

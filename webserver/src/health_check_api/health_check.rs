use axum::{http::StatusCode, Json};
use serde_derive::Serialize;
use tracing::info;

pub async fn health() -> (StatusCode, Json<Status>) {
    info!("health");
    let stat = Status {
        status: 0,
        description: String::from("running"),
    };

    (StatusCode::OK, Json(stat))
}

#[derive(Serialize)]
pub struct Status {
    status: i8,
    description: String,
}


#[tokio::test]
async fn health_check_returns_ok() {
    let (status, body) = health().await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.0.status, 0);
}
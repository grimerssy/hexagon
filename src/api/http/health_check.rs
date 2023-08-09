use axum::{http::StatusCode, routing::get, Router};

use crate::App;

pub fn router() -> Router<App> {
    Router::new().route("/", get(health_check))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::App;

pub fn router() -> Router<App> {
    Router::new()
        .route("/", get(get_all))
        .route("/", post(create))
}

async fn get_all(State(app): State<App>) -> Json<Vec<i32>> {
    Json(app.get_numbers().await.clone())
}

#[derive(Deserialize)]
struct Payload {
    number: i32,
}

async fn create(
    Query(Payload { number }): Query<Payload>,
    State(mut app): State<App>,
) -> StatusCode {
    app.add_number(number).await;
    StatusCode::OK
}

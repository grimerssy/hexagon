use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::domain::Error;

#[derive(Serialize)]
pub struct ErrorMessage {
    error: String,
}

fn status_code(error: &Error) -> StatusCode {
    match error {
        Error::EmailTaken => StatusCode::CONFLICT,
        Error::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = status_code(&self);
        let message = ErrorMessage {
            error: self.to_string(),
        };
        (status_code, Json(message)).into_response()
    }
}

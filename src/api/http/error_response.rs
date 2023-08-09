use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::domain::Error;

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::EmailTaken => StatusCode::CONFLICT,
            Self::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        ErrorResponse {
            status_code: self.status_code(),
            error: self.to_string(),
        }
        .into_response()
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    #[serde(skip)]
    status_code: StatusCode,
    error: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}

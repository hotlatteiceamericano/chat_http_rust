use axum::{http::StatusCode, response::IntoResponse};

/// convert flow anyhow::Error > AppError > IntoResponse
pub struct AppError {
    err: anyhow::Error,
}

// anyhow::Error > AppError
impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self { err: value }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("HTTP Server error: {:?}", self.err);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self.err)).into_response()
    }
}

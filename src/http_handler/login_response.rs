use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub email: String,
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub plain_otp: String,
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

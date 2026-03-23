use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResponse {
    pub websocket_url: String,
    pub jwt_token: String,
}

impl IntoResponse for AuthResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

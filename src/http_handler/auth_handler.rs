use axum::{Json, extract::State};

use crate::{
    app_error::AppError,
    app_state::AppState,
    http_handler::{auth_request::AuthRequest, auth_response::AuthResponse},
};

pub async fn handle(
    State(app_state): State<AppState>,
    Json(request): Json<AuthRequest>,
) -> Result<AuthResponse, AppError> {
}

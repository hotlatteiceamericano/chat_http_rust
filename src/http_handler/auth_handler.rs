use anyhow::Context;
use axum::{Json, extract::State};
use chat_common::auth_response::AuthResponse;
use mongodb::bson;

use crate::{
    app_error::AppError,
    app_state::AppState,
    http_handler::{
        auth_request::AuthRequest,
        jwt,
        otp::{Otp, hash_otp},
    },
};

pub async fn handle(
    State(app_state): State<AppState>,
    Json(request): Json<AuthRequest>,
) -> Result<AuthResponse, AppError> {
    let otps = app_state.db.collection::<Otp>("otps");

    let otp_doc = otps
        .find_one(bson::doc! { "email": &request.email })
        .await
        .context("failed to look up OTP")?;

    let otp_doc = otp_doc.ok_or_else(|| anyhow::anyhow!("no OTP found for this email"))?;

    let submitted_hash = hash_otp(&request.otp);
    if submitted_hash != otp_doc.code_hash {
        return Err(anyhow::anyhow!("invalid OTP").into());
    }

    // OTP matched — delete it so it can't be reused
    otps.delete_one(bson::doc! { "email": &request.email })
        .await
        .context("failed to delete used OTP")?;

    tracing::info!("OTP verified for {}", request.email);

    let jwt_token = jwt::create_token(&request.email, &app_state.jwt_secret)
        .context("failed to create JWT token")?;

    Ok(AuthResponse {
        websocket_url: String::from("ws://0.0.0.0:8081/ws"),
        jwt_token,
    })
}

// todo: add test like login handler

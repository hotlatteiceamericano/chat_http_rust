use anyhow::Context;
use axum::{Json, extract::State};
use mongodb::bson;

use crate::{
    app_error::AppError,
    app_state::AppState,
    http_handler::{
        auth_request::AuthRequest,
        auth_response::AuthResponse,
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

    // TODO: generate a real JWT token
    Ok(AuthResponse {
        websocket_url: String::from("ws://127.0.0.1:3000/ws"),
        jwt_token: String::from("a_valid_jwt_token"),
    })
}

use anyhow::{Context, Result};
use axum::{Json, extract::State, http::StatusCode};
use chat_common::user::User;
use mongodb::{Collection, bson};

use crate::{
    app_error::AppError,
    app_state::AppState,
    http_handler::{
        login_request::LoginRequest,
        otp::Otp,
    },
};

pub async fn handle(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<StatusCode, AppError> {
    let users = app_state.db.collection::<User>("users");
    if let Ok(Some(_user)) = email_look_up(&payload.email, users).await {
        tracing::info!("found user with email: {}", payload.email);
    } else {
        tracing::warn!("email: {} not found, creating one", payload.email);
        let users = app_state.db.collection::<User>("users");
        users
            .insert_one(User::new("Alice", String::from("alice@chat.com")))
            .await
            .context(format!(
                "cannot create a new user for email: {}",
                payload.email
            ))?;
    };

    // Generate OTP, hash it, and store in the otps collection
    let otp = Otp::new(payload.email.clone());

    let otps = app_state.db.collection::<Otp>("otps");

    // Remove any existing OTP for this email before inserting a new one
    otps.delete_many(bson::doc! { "email": &payload.email })
        .await
        .context("failed to clear old OTPs")?;

    // TODO: send OTP via email using lettre. For now, log it.
    tracing::info!("OTP for {}: {}", payload.email, otp.plain_otp());

    otps.insert_one(otp)
        .await
        .context(format!("failed to store OTP for email: {}", payload.email))?;

    Ok(StatusCode::OK)
}

async fn email_look_up(email: &str, users: Collection<User>) -> anyhow::Result<Option<User>> {
    users
        .find_one(bson::doc! {"email": email})
        .await
        .context(format!("failed to find user with email: {}", email))
}

// #[cfg(test)]
// mod test {
//     use axum::{Router, routing::post};
//     use axum_test::TestServer;
//     use rstest::{fixture, rstest};

//     use crate::{app_state::AppState, http_handler::login_handler};
//
//     #[fixture]
//     fn test_server() -> TestServer {
//         // research how to use docker to run cargo test together with a local mongodb
//         let db = get_test_db();
//         let app_state = AppState::new();
//         let app = Router::new()
//             .route("/login", post(login_handler::handle))
//             .with_state(app_state);
//         TestServer::new(app).unwrap()
//     }
//
//     fn get_test_db() -> mongodb::Database {
//         let client = mongodb::Client::with_uri_str("mongodb::")
//     }
//
//     #[rstest]
//     #[tokio::test]
//     async fn test_success_case(test_server: TestServer) {
//         let response = test_server.post("login").await;
//
//         response.assert_status_not_ok();
//     }
// }

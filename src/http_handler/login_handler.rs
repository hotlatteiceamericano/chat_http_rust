use anyhow::{Context, Result};
use axum::{Json, extract::State, http::StatusCode};
use chat_common::user::User;
use mongodb::{Collection, bson};

use crate::{
    app_error::AppError,
    app_state::AppState,
    http_handler::{login_request::LoginRequest, otp::Otp},
};

pub async fn handle(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<StatusCode, AppError> {
    create_if_not_exist(&payload.email, app_state.db.collection::<User>("users")).await?;

    let otp = create_otp(&payload.email, app_state.db.collection::<Otp>("otps")).await?;

    // TODO: send OTP via email using lettre. For now, log it.
    tracing::info!("OTP for {}: {}", payload.email, otp.plain_otp());

    Ok(StatusCode::OK)
}

async fn create_if_not_exist(email: &str, user_collection: Collection<User>) -> anyhow::Result<()> {
    let existing_user = user_collection
        .find_one(bson::doc! {"email": email})
        .await
        .context(format!("failed to find user with email: {}", email));

    if let Ok(None) = existing_user {
        tracing::warn!("email: {} not found, creating one", email);
        user_collection
            .insert_one(User::new("Alice", String::from("alice@chat.com")))
            .await
            .context(format!("cannot create a new user for email: {}", email))
            .context("failed to create new user")?;
    };

    Ok(())
}

async fn create_otp(email: &str, otp_collection: Collection<Otp>) -> anyhow::Result<Otp> {
    let otp = Otp::new(email.to_owned());

    // to only keep the newest otp
    otp_collection
        .delete_many(bson::doc! { "email": &email })
        .await
        .context("failed to clear old OTPs")?;
    otp_collection
        .insert_one(&otp)
        .await
        .context(format!("failed to store OTP for email: {}", email))?;

    Ok(otp)
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

use anyhow::{Context, Result};
use axum::{Json, extract::State, http::StatusCode};
use chat_common::user::User;
use mongodb::{Database, bson};

use crate::{
    app_error::AppError,
    app_state::AppState,
    http_handler::{login_request::LoginRequest, login_response::LoginResponse},
};

pub async fn handle(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<StatusCode, AppError> {
    let Ok(Some(_user)) = email_look_up(&payload.email, app_state.db).await else {
        return Err(anyhow::anyhow!("email not found").into());
    };
    Ok(StatusCode::OK)
}

async fn email_look_up(email: &str, db: Database) -> anyhow::Result<Option<User>> {
    let users = db.collection::<User>("users");
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
//
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

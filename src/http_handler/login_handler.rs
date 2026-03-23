use axum::{Json, extract::State};
use chat_common::user::User;

use crate::{
    app_error::AppError,
    app_state::AppState,
    http_handler::{login_request::LoginRequest, login_response::LoginResponse},
};

pub async fn handle(
    State(_app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<LoginResponse, AppError> {
    let Some(_user) = email_look_up(&payload.email) else {
        return Err(anyhow::anyhow!("email not found").into());
    };
    let Some(()) = otp_look_up(&payload.email) else {
        return Err(anyhow::anyhow!("OTP not matched").into());
    };
    Ok(LoginResponse {
        email: String::from("test email"),
    })
}

fn email_look_up(_email: &str) -> Option<User> {
    // todo: query user from MongoDB Atlas using email
    Some(User::new(0, "Alice"))
}

fn otp_look_up(_email: &str) -> Option<()> {
    // todo: query OTP from MongoDB Atlas
    Some(())
}

#[cfg(test)]
mod test {
    use axum::{Router, routing::post};
    use axum_test::TestServer;
    use rstest::{fixture, rstest};

    use crate::{app_state::AppState, http_handler::login_handler};

    #[fixture]
    fn test_server() -> TestServer {
        let app_state = AppState::new();
        let app = Router::new()
            .route("/login", post(login_handler::handle))
            .with_state(app_state);
        TestServer::new(app).unwrap()
    }

    #[rstest]
    #[tokio::test]
    async fn test_success_case(test_server: TestServer) {
        let response = test_server.post("login").await;

        response.assert_status_not_ok();
    }
}

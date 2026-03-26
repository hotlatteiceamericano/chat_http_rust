use anyhow::{Context, Result};
use axum::{Json, extract::State, http::StatusCode};
use chat_common::user::User;
use mongodb::{Collection, bson};

use crate::{
    app_error::AppError,
    app_state::AppState,
    http_handler::{login_request::LoginRequest, login_response::LoginResponse, otp::Otp},
};

pub async fn handle(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<LoginResponse, AppError> {
    create_if_not_exist(&payload.email, app_state.db.collection::<User>("users")).await?;

    let otp = create_otp(&payload.email, app_state.db.collection::<Otp>("otps")).await?;

    // TODO: send OTP via email using lettre. For now, log it.
    tracing::info!("OTP for {}: {}", payload.email, otp.plain_otp());

    Ok(LoginResponse {
        plain_otp: otp.plain_otp().to_owned(),
    })
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

#[cfg(test)]
mod test {
    use axum::{Router, routing::post};
    use axum_test::TestServer;
    use mongodb::Client;
    use rstest::{fixture, rstest};
    use testcontainers::{GenericImage, runners::AsyncRunner};

    use crate::{
        app_state::AppState,
        http_handler::{login_handler, login_request::LoginRequest, login_response::LoginResponse},
    };

    #[fixture]
    async fn test_server() -> (TestServer, testcontainers::ContainerAsync<GenericImage>) {
        let container = GenericImage::new("mongo", "7")
            .with_exposed_port(27017.into())
            .start()
            .await
            .unwrap();

        let port = container.get_host_port_ipv4(27017).await.unwrap();
        let client = Client::with_uri_str(format!("mongodb://localhost:{port}"))
            .await
            .unwrap();
        let db = client.database("test_db");

        let app_state = AppState::new(db, "test-secret".into());
        let app = Router::new()
            .route("/login", post(login_handler::handle))
            .with_state(app_state);
        let server = TestServer::new(app).unwrap();

        // return container so it stays alive for the duration of the test
        (server, container)
    }

    #[rstest]
    #[tokio::test]
    async fn test_login_success(
        #[future] test_server: (TestServer, testcontainers::ContainerAsync<GenericImage>),
    ) {
        let (server, _container) = test_server.await;

        let response = server
            .post("/login")
            .json(&LoginRequest {
                email: "test@example.com".into(),
            })
            .await;

        let body: LoginResponse = response.json();
        assert!(!body.plain_otp.is_empty());
    }
}

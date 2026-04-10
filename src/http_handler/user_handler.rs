use anyhow::Context;
use axum::{
    Json,
    extract::{Query, State},
};
use chat_common::user::User;
use mongodb::bson;
use serde::Deserialize;

use crate::{app_error::AppError, app_state::AppState};

#[derive(Deserialize)]
pub struct UserQuery {
    pub email: String,
}

pub async fn handle(
    State(app_state): State<AppState>,
    Query(query): Query<UserQuery>,
) -> Result<Json<User>, AppError> {
    let users = app_state.db.collection::<User>("users");

    let user = users
        .find_one(bson::doc! { "email": &query.email })
        .await
        .context(format!("failed to find user with email: {}", query.email))?
        .context(format!("no user found for email: {}", query.email))?;

    tracing::info!("user found with email: {}", &query.email);

    Ok(Json(user))
}

#[cfg(test)]
mod test {
    use axum::{Router, routing::get};
    use axum_test::TestServer;
    use chat_common::user::User;
    use mongodb::Client;
    use rstest::{fixture, rstest};
    use testcontainers::{GenericImage, runners::AsyncRunner};

    use crate::{app_state::AppState, http_handler::user_handler};

    #[fixture]
    async fn test_server() -> (
        TestServer,
        mongodb::Database,
        testcontainers::ContainerAsync<GenericImage>,
    ) {
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

        let app_state = AppState::new(db.clone(), "test-secret".into());
        let app = Router::new()
            .route("/user", get(user_handler::handle))
            .with_state(app_state);
        let server = TestServer::new(app).unwrap();

        (server, db, container)
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_user_by_email_returns_expected_user(
        #[future] test_server: (
            TestServer,
            mongodb::Database,
            testcontainers::ContainerAsync<GenericImage>,
        ),
    ) {
        let (server, db, _container) = test_server.await;

        // seed a user directly into the db
        let email = "test@example.com";
        let display_name = "Test User";
        db.collection::<User>("users")
            .insert_one(User::new(display_name, email.to_string()))
            .await
            .unwrap();

        let response = server.get("/user").add_query_param("email", email).await;

        response.assert_status_ok();
        let body: User = response.json();
        assert_eq!(body.email(), email);
        assert_eq!(body.display_name(), display_name);
        assert!(!body.id().is_empty());
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_user_by_email_not_found(
        #[future] test_server: (
            TestServer,
            mongodb::Database,
            testcontainers::ContainerAsync<GenericImage>,
        ),
    ) {
        let (server, _db, _container) = test_server.await;

        let response = server
            .get("/user")
            .add_query_param("email", "missing@example.com")
            .await;

        response.assert_status_failure();
    }
}

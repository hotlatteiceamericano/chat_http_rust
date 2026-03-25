use axum::{Router, routing::post};
use mongodb::Client;
use tracing_subscriber::EnvFilter;

use crate::{
    app_state::AppState,
    http_handler::{auth_handler, login_handler, otp::Otp},
};
pub mod app_error;
pub mod app_state;
pub mod http_handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let db_uri = std::env::var("MONGODB_URI").expect("MongoDB URI required");
    let client = Client::with_uri_str(db_uri).await.unwrap_or_else(|e| {
        tracing::error!("cannot connected to MongoDB Atlas, error: {:?}", e);
        std::process::exit(1);
    });
    let db = client.database("chat_app_db");

    let app_state = AppState::new(db);

    let app = Router::new()
        .route("/login", post(login_handler::handle))
        .route("/auth", post(auth_handler::handle))
        .with_state(app_state);
    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("chat http server starts");
    axum::serve(listner, app).await.unwrap();
}

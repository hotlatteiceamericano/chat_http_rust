use axum::{Router, routing::post};
use tracing_subscriber::EnvFilter;

use crate::{
    app_state::AppState,
    http_handler::{auth_handler, login_handler},
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

    let app_state = AppState::new();
    let app = Router::new()
        .route("/login", post(login_handler::handle))
        .route("/auth", post(auth_handler::handle))
        .with_state(app_state);
    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("chat http server starts");
    axum::serve(listner, app).await.unwrap();
}

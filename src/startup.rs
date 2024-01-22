use std::sync::Arc;

use crate::{routes::{healthcheck, ping}, settings::Settings};
use axum::{routing::get, Router};
use tera::Tera;
use tower_http::services::ServeDir;

#[derive(Debug)]
pub struct AppState {
    pub tera: Tera,
}
impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(AppState {
            tera: Tera::new("src/templates/**/*.html")?
        })
    }
}

pub type AppStateRC = Arc<AppState>;

pub async fn run(configuration: Settings) -> anyhow::Result<()> {
    tracing::info!(
        "Server is running baby !! Running on http://{}:{}",
        &configuration.application.host,
        &configuration.application.port
    );

    let app_state = AppStateRC::new(AppState::new().await?);

    let app = Router::new()
        .route("/healthcheck", get(healthcheck::healtcheck))
        .route("/", get(ping::ping))
        .with_state(app_state.clone())
        .nest_service("/assets", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        &configuration.application.host, &configuration.application.port
    ))
    .await
    .expect("Failed to open the server.");
    axum::serve(listener, app).await?;
    Ok(())
} 

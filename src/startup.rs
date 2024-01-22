use crate::{routes::healthcheck, settings::Settings};
use axum::{routing::get, Router};

pub async fn run(configuration: Settings) -> anyhow::Result<()> {
    tracing::info!(
        "Server is running baby !! Running on http://{}:{}",
        &configuration.application.host,
        &configuration.application.port
    );

    let app = Router::new().route("/healthcheck", get(healthcheck::healtcheck));

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        &configuration.application.host, &configuration.application.port
    ))
    .await
    .expect("Failed to open the server.");
    axum::serve(listener, app).await?;
    Ok(())
}

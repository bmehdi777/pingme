use crate::startup::AppStateRC;
use axum::{extract::State, http::StatusCode, response::Html};
use tera::Context;

#[tracing::instrument]
pub async fn ping(State(app_state): State<AppStateRC>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();

    match app_state.tera.render("ping.html", &context) {
        Ok(v) => {
            tracing::info!("{}", v.to_string());
            Ok(Html(v.to_string()))
        }
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

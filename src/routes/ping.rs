use std::{sync::Arc, time::Duration};

use crate::startup::AppStateRC;
use axum::{extract::State, http::StatusCode, response::Html};
use ping_rs::{send_ping, PingOptions};
use tera::Context;

#[tracing::instrument]
pub async fn ping(State(app_state): State<AppStateRC>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    context.insert("title", &app_state.settings.target.title);

    let success: bool;
    match ping_destination(app_state.settings.target.address.clone()) {
        Ok(_) => success = true,
        Err(_) => success = false,
    }
    context.insert("ping_success", &success);

    match app_state.tera.render("ping.html", &context) {
        Ok(v) => Ok(Html(v.to_string())),
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn api_ping(State(app_state): State<AppStateRC>) -> StatusCode {
    match ping_destination(app_state.settings.target.address.clone()) {
        Ok(_)=> StatusCode::OK,
        Err(_)=> StatusCode::SERVICE_UNAVAILABLE,
    }
}

#[tracing::instrument]
fn ping_destination(dest: String) -> Result<String, String> {
    let data = [1, 2, 3, 4];
    let timeout = Duration::from_secs(1);
    let options = PingOptions {
        ttl: 128,
        dont_fragment: true,
    };
    tracing::info!("Pinging : {}", dest);
    let result = send_ping(&dest.parse().unwrap(), timeout, &data, Some(&options));
    match result {
        Ok(reply) => {
            tracing::info!("Ping succeed");
            Ok(format!(
                "Reply from {}: bytes={} times={}ms TTL={}",
                reply.address,
                data.len(),
                reply.rtt,
                options.ttl
            ))
        }
        Err(e) => {
            tracing::info!("Ping failed");
            Err(format!("Could not ping the destination : {:?}", e))
        }
    }
}

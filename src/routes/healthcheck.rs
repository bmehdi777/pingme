use axum::http::StatusCode;

pub async fn healtcheck() -> StatusCode {
    StatusCode::OK
}

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn health_router() -> Router {
    Router::new().route("/health", get(health))
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "The server is healthy.")
}

use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use tokio::net::TcpListener;

static SERVER_ADDRESS: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() -> Result<(), ()> {
    let health_router = Router::new().route("/health", get(health));
    let calculate_router = Router::new().route("/calculate", get(calculate));

    let main_router = Router::new().merge(health_router).merge(calculate_router);

    let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();
    println!("----> LISTENING on {:?}\n", listener.local_addr().unwrap());
    axum::serve(listener, main_router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "The server is working.")
}

#[derive(Debug, Deserialize)]
struct CalculateParams {
    name: Option<String>,
}
async fn calculate(Query(params): Query<CalculateParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("");
    Html(format!("Hello <strong>{name}</strong>"))
}

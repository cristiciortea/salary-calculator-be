use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};

use crate::models::calculations::CalculateSchema;
use crate::services::calculations::perform_calculation;
use crate::validators::calculations::validate_calculate_input;

pub fn calculate_router() -> Router {
    Router::new().route("/calculate", post(calculate))
}

pub async fn calculate(Json(data): Json<CalculateSchema>) -> Response {
    println!("->> {:<12} - Calculate handler - {data:?}", "HANDLER");
    let calculation_input = match validate_calculate_input(&data) {
        Ok(data) => data,
        Err(message) => return (StatusCode::BAD_REQUEST, message).into_response(),
    };

    let calculation_results = perform_calculation(calculation_input).await;
    println!(
        "->> {:<12} - Calculate calculation_results - {calculation_results:?}",
        "DEBUG"
    );

    Json(calculation_results).into_response()
}

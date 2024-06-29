use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Json as JsonResponse};
use axum::Router;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

static SERVER_ADDRESS: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() -> Result<(), ()> {
    let health_router = Router::new().route("/health", get(health));
    let calculate_router = Router::new().route("/calculate", post(calculate));

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

#[derive(Debug)]
enum Currency {
    DOLLAR,
    RON,
    EURO,
}

impl Currency {
    fn from_str(currency: &str) -> Option<Currency> {
        match currency.trim().to_uppercase().as_str() {
            "DOLLAR" => Some(Currency::DOLLAR),
            "RON" => Some(Currency::RON),
            "EURO" => Some(Currency::EURO),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CalculateParams {
    salary: Option<String>,
    currency: Option<String>,
    custom_tax: Option<String>,
}

#[derive(Debug, Serialize)]
struct CalculateResults {
    brute_salary: u32,
    net_salary: u32,
    cass: u32,
    cas: u32,
    income_tax: u32,
    personal_deduction: u32,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

async fn calculate(Json(data): Json<CalculateParams>) -> Response {
    println!("->> {:<12} - Calculate handler - {data:?}", "HANDLER");
    let salary: u32 = match data.salary.as_deref().unwrap_or("").trim().parse() {
        Ok(sal) => sal,
        Err(_) => {
            let error_response = ErrorResponse {
                message: "Invalid or missing salary".to_string(),
            };
            return (StatusCode::NOT_ACCEPTABLE, error_response.message).into_response();
        }
    };
    let currency = match data.currency.as_deref().and_then(|c| Currency::from_str(c)) {
        Some(currency) => currency,
        None => {
            let error_response = ErrorResponse {
                message: "Currency not supported.".to_string(),
            };
            return (StatusCode::NOT_ACCEPTABLE, error_response.message).into_response();
        }
    };
    let custom_tax: Option<u32> = data.custom_tax.as_deref()
        .get_or_insert("")
        .trim()
        .parse()
        .ok();

    let calculate_results = CalculateResults {
        brute_salary: salary,
        net_salary: 0,
        cass: 0,
        cas: 0,
        income_tax: 0,
        personal_deduction: 0,
    };

    Json(calculate_results).into_response()
}

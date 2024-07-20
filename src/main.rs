mod db;
mod db_backup;
mod utils;

use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use rusqlite::Connection;

use crate::db::{get_tax_rates, setup_db, TaxRates};
use crate::db_backup::get_current_year;
use crate::utils::round_to;

static SERVER_ADDRESS: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("[INFO]: Current year is {}...", get_current_year());
    println!("[INFO]: Set up the database...");
    let conn = Connection::open("./tax_rates.db").expect("Sqlite conn should be able to open. Cause");
    setup_db(&conn).expect("Setup db should work. Cause");

    println!("[INFO]: Create routers...");
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
    (StatusCode::OK, "The server is healthy.")
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

#[derive(Debug, PartialEq)]
pub enum IncomeType {
    NET,
    BRUTE,
}

impl IncomeType {
    fn from_str(income_type: &str) -> Option<IncomeType> {
        match income_type.trim().to_uppercase().as_str() {
            "NET" => Some(IncomeType::NET),
            "BRUTE" => Some(IncomeType::BRUTE),
            _ => None,
        }
    }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CalculateParams {
    // We have all parameters optional because we want to output
    // custom error messages from validations instead of a typical
    // axum failure message when a parameter is missing.
    income: Option<String>,
    income_type: Option<String>,
    currency: Option<String>,
    custom_tax: Option<String>,
    year: Option<String>,
}

#[derive(Debug)]
struct CalculationInput {
    income: u32,
    income_type: IncomeType,
    currency: Currency,
    custom_tax: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct TaxInfo {
    cas: f64,
    cass: f64,
    dp: Option<f64>,
    income: f64,
    cam: f64,
}

trait Roundable {
    fn apply_rounding(&self, decimals: i32) -> Self;
}


#[derive(Debug, Serialize)]
pub struct CalculationResults {
    brute_income: f64,
    net_income: f64,
    cas: f64,
    cass: f64,
    cam: f64,
    income_tax: f64,
}

impl Default for CalculationResults {
    fn default() -> Self {
        CalculationResults {
            brute_income: 0.0,
            net_income: 0.0,
            cass: 0.0,
            cas: 0.0,
            cam: 0.0,
            income_tax: 0.0,
        }
    }
}

impl Roundable for CalculationResults {
    fn apply_rounding(&self, decimals: i32) -> Self {
        CalculationResults {
            brute_income: round_to(self.brute_income, decimals),
            net_income: round_to(self.net_income, decimals),
            cas: round_to(self.cas, decimals),
            cass: round_to(self.cass, decimals),
            cam: round_to(self.cam, decimals),
            income_tax: round_to(self.income_tax, decimals),
        }
    }
}


fn validate_calculate_input(data: &CalculateParams) -> Result<CalculationInput, String> {
    let income: u32 = match data.income.as_deref().unwrap_or("").trim().parse() {
        Ok(output) => output,
        Err(_) => { return Err(String::from("Invalid or missing income.")); }
    };

    let income_type = match data.income_type.as_deref().and_then(|i| IncomeType::from_str(i)) {
        Some(income_type) => income_type,
        None => {
            return Err(format!("Unsupported income type {:?}.", data.income_type.clone().unwrap_or_default()));
        }
    };

    let currency = match data.currency.as_deref().and_then(|c| Currency::from_str(c)) {
        Some(currency) => currency,
        None => {
            return Err(format!("Currency {:?} not supported.", data.currency.clone().unwrap_or_default()));
        }
    };

    let custom_tax: Option<u32> = data.custom_tax.as_deref()
        .get_or_insert("")
        .trim()
        .parse()
        .ok();

    Ok(CalculationInput { income, income_type, currency, custom_tax })
}

async fn perform_calculation(input: CalculationInput) -> CalculationResults {
    // The main function where the calculation works.
    println!("->> {:<12} - Calculate calculation_input - {input:?}", "DEBUG in perform_calculation");
    let conn = Connection::open("./tax_rates.db").expect("Sqlite conn should be able to open. Cause");
    let tax_rates = match get_tax_rates(&conn, get_current_year()) {
        None => { return CalculationResults::default(); }
        Some(tax_rates) => tax_rates
    };

    if input.income_type == IncomeType::NET {
        let net_income = input.income as f64;
        // net_income = taxable_income - calculated_income_tax
        // net_income = (net_income + calculated_income_tax) - calculated_income_tax
        let brute_income = net_income / ((1.0 - tax_rates.social_security - tax_rates.health_insurance) * (1.0 - tax_rates.income_tax));

        let calculated_cas = brute_income * tax_rates.social_security;
        let calculated_cass = brute_income * tax_rates.health_insurance;
        let calculated_cam_tax = brute_income * tax_rates.insurance_contribution;
        let taxable_income = brute_income - calculated_cas - calculated_cass;
        let calculated_income_tax = taxable_income * tax_rates.income_tax;
        CalculationResults {
            net_income,
            brute_income,
            cas: calculated_cas,
            cass: calculated_cass,
            cam: calculated_cam_tax,
            income_tax: calculated_income_tax,
        }.apply_rounding(2)
    } else {
        let brute_income = input.income as f64;
        let calculated_cas = brute_income * tax_rates.social_security;
        let calculated_cass = brute_income * tax_rates.health_insurance;
        let taxable_income = brute_income - calculated_cas - calculated_cass;
        let calculated_cam_tax = brute_income * tax_rates.insurance_contribution;
        let calculated_income_tax = taxable_income * tax_rates.income_tax;
        let net_income = taxable_income - calculated_income_tax;
        CalculationResults {
            brute_income,
            net_income,
            cas: calculated_cas,
            cass: calculated_cass,
            cam: calculated_cam_tax,
            income_tax: calculated_income_tax,
        }.apply_rounding(2)
    }
}

async fn calculate(Json(data): Json<CalculateParams>) -> Response {
    println!("->> {:<12} - Calculate handler - {data:?}", "HANDLER");
    let calculation_input = match validate_calculate_input(&data) {
        Ok(data) => data,
        Err(message) => { return (StatusCode::BAD_REQUEST, message).into_response() }
    };

    let calculation_results = perform_calculation(calculation_input).await;
    println!("->> {:<12} - Calculate calculation_results - {calculation_results:?}", "DEBUG");

    Json(calculation_results).into_response()
}

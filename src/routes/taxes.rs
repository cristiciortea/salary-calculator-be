use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use rusqlite::Connection;

use crate::database::db::get_tax_rates;
use crate::database::db_backup::get_current_year;
use crate::models::calculations::TaxInfo;

pub fn taxes_router() -> Router {
    Router::new().route("/taxes", get(fetch_current_year_tax_rates))
}

pub async fn fetch_current_year_tax_rates() -> Response {
    let conn = Connection::open("./tax_rates.db")
        .expect("Sqlite conn should be able to open. Error cause");
    let tax_rates = get_tax_rates(&conn, get_current_year())
        .expect("Tax rates for current year should be found in the database. Error cause");
    let tax_info = TaxInfo {
        year: &tax_rates.year,
        cas: &tax_rates.health_insurance,
        cass: &tax_rates.social_security,
        income: &tax_rates.income_tax,
        cam: &tax_rates.insurance_contribution,
        dp: None,
    };

    Json(tax_info).into_response()
}

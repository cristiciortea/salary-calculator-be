use rusqlite::Connection;
use crate::database::db::get_tax_rates;
use crate::database::db_backup::get_current_year;
use crate::models::calculations::{CalculationInput, CalculationResults, IncomeType};

pub async fn perform_calculation(input: CalculationInput) -> CalculationResults {
    // The main function where the calculation works.
    println!("->> {:<12} - Calculate calculation_input - {input:?}", "DEBUG in perform_calculation");
    let conn = Connection::open("./tax_rates.db").expect("Sqlite conn should be able to open. Cause");
    let tax_rates = match get_tax_rates(&conn, get_current_year()) {
        None => { return CalculationResults::default(); }
        Some(tax_rates) => tax_rates
    };

    if input.income_type == IncomeType::NET {
        let net_income = input.income as f64;
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
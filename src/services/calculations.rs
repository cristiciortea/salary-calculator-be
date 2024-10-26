use crate::database::db::get_tax_rates;
use crate::database::db_backup::get_current_year;
use crate::models::calculations::{CalculationInput, CalculationResults, IncomeType};
use rusqlite::Connection;

pub async fn perform_calculation(input: CalculationInput) -> CalculationResults {
    // The main function where the calculation works.
    println!(
        "->> {:<12} - Calculate calculation_input - {input:?}",
        "DEBUG in perform_calculation"
    );
    let conn =
        Connection::open("./tax_rates.db").expect("Sqlite conn should be able to open. Cause");
    let tax_rates = get_tax_rates(&conn, input.year.unwrap_or_else(get_current_year))
        .expect("Tax rates for current year should be found in the database. Cause");

    if input.income_type == IncomeType::NET {
        let net_income = input.income as f64;
        let brute_income = net_income
            / ((1.0 - tax_rates.social_security - tax_rates.health_insurance)
                * (1.0 - tax_rates.income_tax));

        let calculated_cas = brute_income * tax_rates.social_security;
        let calculated_cass = brute_income * tax_rates.health_insurance;
        let calculated_cam_tax = brute_income * tax_rates.insurance_contribution;
        let taxable_income = brute_income - calculated_cas - calculated_cass;
        let calculated_income_tax = taxable_income * tax_rates.income_tax;
        let total_salary = brute_income + calculated_cam_tax;
        CalculationResults {
            net_income,
            brute_income,
            cas: calculated_cas,
            cass: calculated_cass,
            income_tax: calculated_income_tax,
            cam: calculated_cam_tax,
            total_salary: brute_income + calculated_cam_tax,
            employee_tax_percentage: net_income * 100f64 / total_salary,
            state_tax_percentage: ((total_salary - net_income) * 100f64) / total_salary,
        }
        .apply_rounding(2)
    } else {
        let brute_income = input.income as f64;
        let calculated_cas = brute_income * tax_rates.social_security;
        let calculated_cass = brute_income * tax_rates.health_insurance;
        let taxable_income = brute_income - calculated_cas - calculated_cass;
        let calculated_cam_tax = brute_income * tax_rates.insurance_contribution;
        let calculated_income_tax = taxable_income * tax_rates.income_tax;
        let net_income = taxable_income - calculated_income_tax;
        let total_salary = brute_income + calculated_cam_tax;
        CalculationResults {
            brute_income,
            net_income,
            total_salary,
            cas: calculated_cas,
            cass: calculated_cass,
            income_tax: calculated_income_tax,
            cam: calculated_cam_tax,
            employee_tax_percentage: (net_income * 100f64 / total_salary),
            state_tax_percentage: ((total_salary - net_income) * 100f64) / total_salary,
        }
        .apply_rounding(2)
    }
}

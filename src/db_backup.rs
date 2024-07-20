use chrono::{Datelike, Local};
pub fn get_current_year() -> i32 {
    let dt = Local::now();
    dt.year()
}

pub fn get_initial_insert_statements() -> Vec<String> {
    vec![
        // 2025.
        format!("INSERT INTO tax_rates (year, income_tax, social_security, health_insurance, insurance_contribution)
              VALUES ({}, 0.10, 0.25, 0.1, 0.0225)
              ON CONFLICT(year) DO NOTHING;", get_current_year()),
        // 2024.
        "INSERT INTO tax_rates (year, income_tax, social_security, health_insurance, insurance_contribution)
              VALUES (2024, 0.10, 0.25, 0.1, 0.0225)
              ON CONFLICT(year) DO NOTHING;".to_string(),
        // 2023.
        "INSERT INTO tax_rates (year, income_tax, social_security, health_insurance, insurance_contribution)
              VALUES (2023, 0.10, 0.25, 0.1, 0.0225)
              ON CONFLICT(year) DO NOTHING;".to_string(),
    ]
}
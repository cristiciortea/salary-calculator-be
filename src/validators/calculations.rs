use crate::models::calculations::{CalculateSchema, CalculationInput, Currency, IncomeType};

pub fn validate_calculate_input(data: &CalculateSchema) -> Result<CalculationInput, String> {
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
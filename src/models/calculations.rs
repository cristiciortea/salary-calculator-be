use crate::utils::round_to;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Currency {
    DOLLAR,
    RON,
    EURO,
}

impl Currency {
    pub(crate) fn from_str(currency: &str) -> Option<Currency> {
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
    pub(crate) fn from_str(income_type: &str) -> Option<IncomeType> {
        match income_type.trim().to_uppercase().as_str() {
            "NET" => Some(IncomeType::NET),
            "BRUTE" => Some(IncomeType::BRUTE),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculateSchema {
    // We have all parameters optional because we want to output
    // custom error messages from validations instead of a typical
    // axum failure message when a parameter is missing.
    pub income: Option<String>,
    pub income_type: Option<String>,
    pub currency: Option<String>,
    pub custom_tax: Option<String>,
    pub year: Option<String>,
}

#[derive(Debug)]
pub struct CalculationInput {
    pub income: u32,
    pub income_type: IncomeType,
    pub currency: Currency,
    pub custom_tax: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct TaxInfo<'a> {
    pub cas: &'a f64,
    pub cass: &'a f64,
    pub income: &'a f64,
    pub cam: &'a f64,
    pub dp: Option<&'a f64>,
}

#[derive(Debug, Serialize)]
pub struct CalculationResults {
    pub brute_income: f64,
    pub net_income: f64,
    pub cas: f64,
    pub cass: f64,
    pub cam: f64,
    pub income_tax: f64,
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

impl CalculationResults {
    pub fn apply_rounding(&self, decimals: i32) -> Self {
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

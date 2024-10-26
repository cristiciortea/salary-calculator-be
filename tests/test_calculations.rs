use anyhow::Result;
use axum::http::StatusCode;
use common::LOCALHOST;
use serde::Deserialize;
use serde_json::json;
mod common;

#[tokio::test]
async fn check_health() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8000")?;
    let response = client.do_get("/health").await?;
    response.print().await?;

    let status = response.status();
    let message = response.text_body()?;
    assert_eq!(status, StatusCode::OK.as_u16());
    assert_eq!(message, "The server is healthy.");
    Ok(())
}

#[tokio::test]
async fn calculate_with_wrong_currency_should_respond_error_400() {
    let client = reqwest::Client::new();
    let data = json!({
        "income": "1000",
        "incomeType": "brute",
        "currency": "YEN", // Invalid currency.
        "customTax": "500",
    });

    let response = client
        .post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST.as_u16());
    let text = response.text().await.expect("Failed to read response body");
    assert!(text.contains("Currency \"YEN\" not supported"));
}

#[tokio::test]
async fn calculate_with_wrong_income_type_should_respond_error_400() {
    let client = reqwest::Client::new();
    let data = json!({
        "income": "1000",
        "incomeType": "wrong-income-type", // this is invalid.
        "currency": "RON",
        "customTax": "500",
    });

    let response = client
        .post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST.as_u16());
    let text = response.text().await.expect("Failed to read response body");
    assert!(text.contains("Unsupported income type \"wrong-income-type\""));
}

#[tokio::test]
async fn calculate_with_empty_currency_should_respond_error_400() {
    let client = reqwest::Client::new();
    let data = json!({
        "income": "1000",
        "incomeType": "net",
        "currency": null,
        "customTax": "500",
    });

    let response = client
        .post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST.as_u16());
    let text = response.text().await.expect("Failed to read response body");
    assert!(text.contains("Currency \"\" not supported"));
}

#[tokio::test]
async fn calculate_missing_salary_should_respond_400() {
    let client = reqwest::Client::new();
    let data = json!({
        // Missing "salary"
        "currency": "RON",
        "customTax": "500",
    });

    let response = client
        .post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST.as_u16());
    let text = response.text().await.unwrap();
    assert!(text.contains("Invalid or missing income."));
}

#[derive(Debug, Deserialize)]
struct CalculationResponse {
    brute_income: f64,
    net_income: f64,
    cas: f64,
    cass: f64,
    income_tax: f64,
    cam: f64,
    total_salary: f64,
    employee_tax_percentage: f64,
    state_tax_percentage: f64,
}

#[tokio::test]
async fn calculate_net_salary_happy_path() -> Result<()> {
    let client = reqwest::Client::new();
    let data = json!({
        "income": "10000",
        "incomeType": "net",
        "currency": "ron",
        "customTax": null,
    });
    let response = client
        .post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await?;
    let status = response.status();
    let response: CalculationResponse = response.json().await?;

    assert_eq!(status, StatusCode::OK.as_u16());
    assert_eq!(response.brute_income, 17094.02);
    assert_eq!(response.net_income, 10000.0);
    assert_eq!(response.cas, 4273.5);
    assert_eq!(response.cass, 1709.4);
    assert_eq!(response.income_tax, 1111.11);
    assert_eq!(response.cam, 384.62);
    assert_eq!(response.total_salary, 17478.63);
    assert_eq!(response.employee_tax_percentage, 57.21);
    assert_eq!(response.state_tax_percentage, 42.79);

    Ok(())
}

#[tokio::test]
async fn calculate_brute_salary_happy_path() -> Result<()> {
    let client = reqwest::Client::new();
    let data = json!({
        "income": "10000",
        "incomeType": "brute",
        "currency": "ron",
        "customTax": null,
    });
    let response = client
        .post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await?;
    let status = response.status();
    let response: CalculationResponse = response.json().await?;

    assert_eq!(status, StatusCode::OK.as_u16());
    assert_eq!(response.brute_income, 10000.0);
    assert_eq!(response.net_income, 5850.0);
    assert_eq!(response.cas, 2500.0);
    assert_eq!(response.cass, 1000.0);
    assert_eq!(response.income_tax, 650.0);
    assert_eq!(response.cam, 225.0);
    assert_eq!(response.total_salary, 10225.0);
    assert_eq!(response.employee_tax_percentage, 57.21);
    assert_eq!(response.state_tax_percentage, 42.79);

    Ok(())
}

#[tokio::test]
async fn calculate_brute_salary_works_for_year_2023() -> Result<()> {
    let client = reqwest::Client::new();
    let data = json!({
        "income": "10000",
        "incomeType": "brute",
        "currency": "ron",
        "customTax": null,
        "year": "2023",
    });
    let response = client
        .post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await?;
    let status = response.status();
    let response: CalculationResponse = response.json().await?;

    assert_eq!(status, StatusCode::OK.as_u16());
    assert_eq!(response.brute_income, 10000.0);
    assert_eq!(response.net_income, 5850.0);
    assert_eq!(response.cas, 2500.0);
    assert_eq!(response.cass, 1000.0);
    assert_eq!(response.income_tax, 650.0);
    assert_eq!(response.cam, 225.0);
    assert_eq!(response.total_salary, 10225.0);
    assert_eq!(response.employee_tax_percentage, 57.21);
    assert_eq!(response.state_tax_percentage, 42.79);

    Ok(())
}

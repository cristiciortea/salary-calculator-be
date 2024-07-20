use anyhow::Result;
use axum::http::StatusCode;
use serde_json::{json};

static LOCALHOST: &str = "http://localhost:8000";

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
async fn calculate_with_wrong_currency_should_return_error_400() {
    let client = reqwest::Client::new();
    let data = json!({
        "income": "1000",
        "incomeType": "brute",
        "currency": "YEN", // Invalid currency.
        "customTax": "500",
    });

    let response = client.post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST.as_u16());
    let text = response.text().await.expect("Failed to read response body");
    assert!(text.contains("Currency \"YEN\" not supported"));
}

#[tokio::test]
async fn calculate_with_wrong_income_type_should_return_error_400() {
    let client = reqwest::Client::new();
    let data = json!({
        "income": "1000",
        "incomeType": "wrong-income-type", // this is invalid.
        "currency": "RON",
        "customTax": "500",
    });

    let response = client.post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST.as_u16());
    let text = response.text().await.expect("Failed to read response body");
    assert!(text.contains("Unsupported income type \"wrong-income-type\""));
}

#[tokio::test]
async fn calculate_with_empty_currency_should_return_error_400() {
    let client = reqwest::Client::new();
    let data = json!({
        "income": "1000",
        "incomeType": "net",
        "currency": null,
        "customTax": "500",
    });

    let response = client.post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST.as_u16());
    let text = response.text().await.expect("Failed to read response body");
    assert!(text.contains("Currency \"\" not supported"));
}

#[tokio::test]
async fn calculate_missing_salary() {
    let client = reqwest::Client::new();
    let data = json!({
        // Missing "salary"
        "currency": "RON",
        "customTax": "500",
    });

    let response = client.post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST.as_u16());
    let text = response.text().await.unwrap();
    assert!(text.contains("Invalid or missing income."));
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
    let response = client.post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await?;
    let status = response.status();
    let text = response.text().await.unwrap();
    println!("DEBUG: {text}");
    assert_eq!(status, StatusCode::OK.as_u16());

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
    let response = client.post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await?;
    let status = response.status();
    let text = response.text().await.unwrap();
    println!("DEBUG: {text}");
    assert_eq!(status, StatusCode::OK.as_u16());

    Ok(())
}
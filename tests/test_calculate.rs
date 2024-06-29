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
    assert_eq!(message, "The server is working.");
    Ok(())
}


#[tokio::test]
async fn calculate_with_invalid_currency_should_return_error_400() {
    let client = reqwest::Client::new();
    let data = json!({
        "salary": "1000",
        "currency": "YEN", // Invalid currency.
        "customTax": "500",
    });

    let response = client.post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE.as_u16());
    let text = response.text().await.expect("Failed to read response body");
    assert!(text.contains("Currency not supported"));
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

    assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE.as_u16());
    let text = response.text().await.unwrap();
    assert!(text.contains("Invalid or missing salary"));
}

#[tokio::test]
async fn calculate_salary_happy_path() -> Result<()> {
    let client = reqwest::Client::new();
    let data = json!({
        "salary": "1000",
        "currency": "dollar",
        "customTax": "abc",
    });
    let response = client.post(format!("{LOCALHOST}/calculate"))
        .json(&data)
        .send()
        .await?;
    let status = response.status();
    let text = response.text().await.unwrap();
    println!("{:?}", text);
    assert_eq!(status, StatusCode::OK.as_u16());

    Ok(())
}
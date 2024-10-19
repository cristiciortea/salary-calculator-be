mod common;

use anyhow::Result;
use axum::http::StatusCode;
use common::LOCALHOST;
use reqwest;
use std::ops::Deref;

#[tokio::test]
async fn fetch_current_taxes_happy_path() -> Result<()> {
    let client = reqwest::Client::new();

    let response = client.get(format!("{LOCALHOST}/taxes")).send().await?;

    let status = response.status();
    let tax_info: serde_json::Value = response.json().await?;

    assert_eq!(status, StatusCode::OK);
    assert!(tax_info.get("cas").is_some());
    assert!(tax_info.get("cass").is_some());
    assert!(tax_info.get("income").is_some());
    assert!(tax_info.get("cam").is_some());
    assert!(tax_info.get("dp").unwrap().is_null());

    Ok(())
}

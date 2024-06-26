use anyhow::Result;
use axum::http::StatusCode;

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
async fn calculate_salary() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8000")?;
    let response = client.do_get("/calculate?name=Axum").await?;
    response.print().await?;
    Ok(())
}

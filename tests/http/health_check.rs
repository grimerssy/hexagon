#[tokio::test]
async fn server_responds() -> anyhow::Result<()> {
    crate::start_server().await?;
    reqwest::get("http://localhost:8080/api/health_check")
        .await?
        .error_for_status()?;
    Ok(())
}

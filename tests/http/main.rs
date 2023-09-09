use hexagon::{init_config, telemetry, App, HttpServer};

mod health_check;

async fn start_server() -> anyhow::Result<()> {
    telemetry::init_test();
    let app_config = init_config()?;
    let http_config = init_config()?;
    let app = App::new(app_config).await?;
    tokio::spawn(HttpServer::run(http_config, app));
    Ok(())
}

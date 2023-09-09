use hexagon::{init_config, telemetry, App, HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init()?;
    let app_config = init_config()?;
    let http_config = init_config()?;
    let app = App::new(app_config).await?;
    HttpServer::run(http_config, app).await
}

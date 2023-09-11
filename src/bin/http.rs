use hexagon::{
    api::HttpServer, config::init_config, telemetry::init_telemetry, App,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_telemetry()?;
    let app_config = init_config()?;
    let app = App::new(app_config).await?;
    let http_config = init_config()?;
    let http_server = HttpServer::new(http_config, app)?;
    http_server.start().await
}

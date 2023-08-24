use hexagon::{init_config, init_telemetry, App, HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = init_config()?;
    let app = App::new(init_config()?)?;
    init_telemetry()?;
    HttpServer::run(config, app).await
}

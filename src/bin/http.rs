use hexagon::{init_config, init_telemetry, HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = init_config()?;
    let app = hexagon::App::new(init_config()?)?;
    init_telemetry()?;
    HttpServer::run(config, app).await
}

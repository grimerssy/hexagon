use hexagon::{init_telemetry, Api, Config, HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init()?;
    init_telemetry()?;
    HttpServer::run(config).await
}

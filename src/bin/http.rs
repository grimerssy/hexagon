use hexagon::{telemetry, Api, Config, HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init()?;
    telemetry::init()?;
    HttpServer::run(config).await
}

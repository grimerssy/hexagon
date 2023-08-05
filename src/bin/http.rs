use hexagon::{Api, Config, HttpServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init()?;
    HttpServer::run(config).await
}

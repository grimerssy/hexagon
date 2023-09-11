#![cfg(not(feature = "skip-io-tests"))]

use std::net::SocketAddr;

use hexagon::{
    api::HttpServer, config::init_config, telemetry::init_test_telemetry, App,
};
use reqwest::{Client, Method, RequestBuilder};

mod health_check;

struct TestServer {
    address: SocketAddr,
    http_client: Client,
}

impl TestServer {
    async fn start() -> anyhow::Result<Self> {
        init_test_telemetry();
        let app_config = init_config()?;
        let http_config = init_config()?;
        let app = App::new(app_config).await?;
        let http_server = HttpServer::new(http_config, app)?;
        let address = http_server.addr()?;
        let http_client = Client::new();
        tokio::spawn(http_server.start());
        Ok(Self {
            address,
            http_client,
        })
    }

    fn call(&self, method: Method, endpoint: &str) -> RequestBuilder {
        let url = self.endpoint_url(endpoint);
        self.http_client.request(method, url)
    }

    fn endpoint_url(&self, endpoint: &str) -> String {
        format!("http://{}{}", self.address, endpoint)
    }
}

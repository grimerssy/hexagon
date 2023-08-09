mod error_response;
mod health_check;

use anyhow::Context;
use axum::Router;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with::serde_as;

use crate::App;

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct HttpConfig {
    pub host: [u8; 4],
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HttpServerConfig {
    pub http: HttpConfig,
}

pub struct HttpServer;

impl HttpServer {
    pub async fn run(config: HttpServerConfig, app: App) -> anyhow::Result<()> {
        let config = config.http;
        let router = Router::new()
            .nest("/health_check", health_check::router())
            .with_state(app);
        let addr = std::net::SocketAddr::from((config.host, config.port));
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .context("Failed to bind to address")
    }
}

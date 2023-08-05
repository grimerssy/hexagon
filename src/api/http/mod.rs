mod numbers;

use anyhow::Context;
use async_trait::async_trait;
use axum::Router;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with::serde_as;

use crate::{services::Service, App, Config};

use super::Api;

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct HttpConfig {
    pub host: [u8; 4],
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    http: HttpConfig,
}

pub struct HttpServer;

#[async_trait]
impl Api for HttpServer {
    type Config = ServerConfig;

    async fn run(config: Config<Self, App>) -> anyhow::Result<()> {
        let app = App::new(config.app)?;
        let router = Router::new()
            .nest("/numbers", numbers::router())
            .with_state(app);
        let addr = std::net::SocketAddr::from((
            config.api.http.host,
            config.api.http.port,
        ));
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .context("Failed to bind to address")
    }
}

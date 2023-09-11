mod error_response;
mod health_check;

use std::net::{SocketAddr, TcpListener};

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

pub struct HttpServer {
    listener: TcpListener,
    router: Router,
}

impl HttpServer {
    #[tracing::instrument(skip(app))]
    pub fn new(config: HttpServerConfig, app: App) -> anyhow::Result<Self> {
        let config = config.http;
        let router = router().with_state(app);
        let addr = SocketAddr::from((config.host, config.port));
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener, router })
    }

    #[tracing::instrument(skip(self))]
    pub async fn start(self) -> anyhow::Result<()> {
        axum::Server::from_tcp(self.listener)?
            .serve(self.router.into_make_service())
            .await?;
        Ok(())
    }

    pub fn addr(&self) -> anyhow::Result<SocketAddr> {
        self.listener
            .local_addr()
            .context("Failed to get TCP listener local address")
    }
}

fn router() -> Router<App> {
    let api_router =
        Router::new().nest("/health_check", health_check::router());
    Router::new().nest("/api", api_router)
}

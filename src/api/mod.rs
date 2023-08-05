mod http;

use std::fmt::Debug;

pub use http::HttpServer;

use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::{App, Config};

#[async_trait]
pub trait Api: Sized {
    type Config: Clone + Debug + DeserializeOwned;

    async fn run(config: Config<Self, App>) -> anyhow::Result<()>;
}

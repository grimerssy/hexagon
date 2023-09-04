mod database;

use async_trait::async_trait;
pub use database::*;

use std::fmt::Debug;

use serde::de::DeserializeOwned;

#[async_trait]
pub trait Service: Clone + Sized {
    type Config: Clone + Debug + DeserializeOwned;

    async fn new(config: Self::Config) -> anyhow::Result<Self>;
}

mod database;

pub use database::*;

use std::fmt::Debug;

use serde::de::DeserializeOwned;

pub trait Service: Clone + Sized {
    type Config: Clone + Debug + DeserializeOwned;

    fn new(config: Self::Config) -> anyhow::Result<Self>;
}

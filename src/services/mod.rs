mod database;

use std::fmt::Debug;

pub use database::*;

use serde::de::DeserializeOwned;

pub trait Service: Clone + Sized {
    type Config: Clone + Debug + DeserializeOwned;

    fn new(config: Self::Config) -> anyhow::Result<Self>;
}

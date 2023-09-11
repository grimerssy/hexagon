mod users;

use async_trait::async_trait;

use crate::{
    domain::User,
    ports::{Database, Service},
};

#[derive(Clone, Default)]
pub struct InMemoryDatabase {
    users: Vec<User>,
}

#[async_trait]
impl Service for InMemoryDatabase {
    type Config = ();

    async fn new(_: Self::Config) -> anyhow::Result<Self> {
        Ok(Self::default())
    }
}

impl Database for InMemoryDatabase {}

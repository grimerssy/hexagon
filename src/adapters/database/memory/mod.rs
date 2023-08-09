mod users;

use crate::{
    domain::User,
    ports::{Database, Service},
};

#[derive(Clone)]
pub struct InMemoryDatabase {
    users: Vec<User>,
}

impl Service for InMemoryDatabase {
    type Config = ();

    fn new(_: Self::Config) -> anyhow::Result<Self> {
        Ok(Self {
            users: Default::default(),
        })
    }
}

impl Database for InMemoryDatabase {}

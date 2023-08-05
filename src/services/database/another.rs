use async_trait::async_trait;

use super::{Database, Service};

#[derive(Clone)]
pub struct AnotherDatabase {
    numbers: Vec<i32>,
}

impl Service for AnotherDatabase {
    type Config = ();

    fn new(_: Self::Config) -> anyhow::Result<Self> {
        Ok(Self {
            numbers: Vec::new(),
        })
    }
}

#[async_trait]
impl Database for AnotherDatabase {
    async fn get_all_numbers(&self) -> &Vec<i32> {
        &self.numbers
    }

    async fn add_number(&mut self, number: i32) {
        self.numbers.push(number);
    }
}

mod another;
mod in_memory;

pub use another::AnotherDatabase;
use async_trait::async_trait;
pub use in_memory::InMemoryDatabase;

use super::Service;

#[async_trait]
pub trait Database: Service {
    async fn get_all_numbers(&self) -> &Vec<i32>;
    async fn add_number(&mut self, number: i32);
}

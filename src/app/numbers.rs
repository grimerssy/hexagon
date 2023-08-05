use crate::services::Database;

use super::GenericApp;

impl<DB> GenericApp<DB>
where
    DB: Database,
{
    pub async fn get_numbers(&self) -> &Vec<i32> {
        self.database.get_all_numbers().await
    }

    pub async fn add_number(&mut self, number: i32) {
        self.database.add_number(number).await;
    }
}

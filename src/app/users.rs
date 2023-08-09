use crate::{
    domain::{NewUser, Result},
    ports::Database,
};

use super::App;

impl<DB> App<DB>
where
    DB: Database,
{
    pub async fn create_user(&mut self, user: NewUser) -> Result<()> {
        self.database.create_user(user).await
    }
}

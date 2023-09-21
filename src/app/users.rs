use crate::{
    domain::{error::Result, user::NewUser},
    ports::database::Database,
};

use super::App;

impl<DB> App<DB>
where
    DB: Database,
{
    #[tracing::instrument(skip(self))]
    pub async fn create_user(&mut self, user: NewUser) -> Result<()> {
        self.database.create_user(&user).await
    }
}

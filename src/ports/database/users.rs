use async_trait::async_trait;

use crate::domain::{NewUser, Result};

#[async_trait]
pub trait UsersDatabase {
    async fn create_user(&mut self, user: NewUser) -> Result<()>;
}
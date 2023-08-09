use async_trait::async_trait;

use crate::{
    domain::{Error, NewUser, Result, User},
    ports::UsersDatabase,
};

use super::InMemoryDatabase;

#[async_trait]
impl UsersDatabase for InMemoryDatabase {
    #[tracing::instrument(name = "Create user in the database", skip(self))]
    async fn create_user(&mut self, user: NewUser) -> Result<()> {
        if self.users.iter().any(|u| u.email == user.email) {
            return Err(Error::EmailTaken);
        }
        let user = User {
            id: self.users.len() as u64 + 1,
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            verification_token: user.verification_token,
            verified: user.verified,
            refresh_token: user.refresh_token,
        };
        self.users.push(user);
        Ok(())
    }
}

use async_trait::async_trait;

use crate::{
    domain::{Error, NewUser, Result, User},
    ports::UserDatabase,
};

use super::InMemoryDatabase;

#[async_trait]
impl UserDatabase for InMemoryDatabase {
    #[tracing::instrument(skip(self), err(level = "warn", Debug))]
    async fn create_user(&mut self, user: &NewUser) -> Result<()> {
        if self.users.iter().any(|u| u.email == user.email) {
            return Err(Error::EmailTaken);
        }
        let user = User {
            id: self.users.len() as u64 + 1,
            name: user.name.clone(),
            email: user.email.clone(),
            password_hash: user.password_hash.clone(),
            verification_token: user.verification_token.clone(),
            verified: user.verified,
            refresh_token: user.refresh_token.clone(),
        };
        self.users.push(user);
        Ok(())
    }
}

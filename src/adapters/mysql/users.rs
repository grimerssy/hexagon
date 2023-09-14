use anyhow::Context;
use async_trait::async_trait;
use secrecy::ExposeSecret;
use sqlx::error::{DatabaseError, ErrorKind};

use crate::{
    domain::{Error, NewUser, Result},
    ports::UsersDatabase,
    telemetry,
};

use super::MySqlDatabase;

#[async_trait]
impl UsersDatabase for MySqlDatabase {
    #[tracing::instrument(skip(self))]
    async fn create_user(&mut self, user: NewUser) -> Result<()> {
        match sqlx::query!(
            "
            insert into users (
              name,
              email,
              password_hash,
              verification_token,
              verified,
              refresh_token
            )
            values (?, ?, ?, ?, ?, ?);
            ",
            user.name,
            user.email,
            user.password_hash.expose_secret(),
            user.verification_token.expose_secret().to_string(),
            user.verified,
            user.refresh_token.expose_secret()
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(e)
                if e.as_database_error()
                    .map(DatabaseError::kind)
                    .is_some_and(|k| k == ErrorKind::UniqueViolation) =>
            {
                Err(Error::EmailTaken).map_err(telemetry::warn)
            }
            Err(e) => Err(e)
                .context("Failed to insert user")
                .map_err(Error::from)
                .map_err(telemetry::error),
        }
    }
}

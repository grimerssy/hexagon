use anyhow::Context;
use async_trait::async_trait;
use secrecy::ExposeSecret;

use crate::{
    domain::{Error, NewUser, Result},
    ports::UsersDatabase,
    telemetry,
};

use super::PostgresqlDatabase;

#[async_trait]
impl UsersDatabase for PostgresqlDatabase {
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
            values ($1, $2, $3, $4, $5, $6)
            on conflict do nothing;
            ",
            user.name,
            user.email,
            user.password_hash.expose_secret(),
            user.verification_token.expose_secret().uuid(),
            user.verified,
            user.refresh_token.expose_secret()
        )
        .execute(&self.pool)
        .await
        .context("Failed to insert user")
        .map_err(telemetry::error)?
        .rows_affected()
        {
            0 => Err(Error::EmailTaken).map_err(telemetry::warn),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }
}

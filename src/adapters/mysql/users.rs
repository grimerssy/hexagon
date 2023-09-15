use anyhow::Context;
use async_trait::async_trait;
use secrecy::ExposeSecret;

use crate::{
    domain::{Error, NewUser, Result},
    ports::UserDatabase,
    telemetry,
};

use super::{is_unique_violation, MySqlDatabase};

#[async_trait]
impl UserDatabase for MySqlDatabase {
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
            Err(e) if is_unique_violation(&e) => {
                Err(Error::EmailTaken).map_err(telemetry::warn)
            }
            Err(e) => Err(e)
                .context("Failed to insert user")
                .map_err(Error::from)
                .map_err(telemetry::error),
        }
    }
}

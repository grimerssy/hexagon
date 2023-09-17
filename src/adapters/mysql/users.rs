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
    async fn create_user(&mut self, user: &NewUser) -> Result<()> {
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
                .map_err(Error::Unexpected)
                .map_err(telemetry::error),
        }
    }
}

#[cfg(test)]
#[cfg(not(feature = "skip-io-tests"))]
mod tests {
    use secrecy::Secret;
    use sqlx::{MySql, Pool};

    use crate::{
        adapters::MySqlDatabase,
        domain::{Error, NewUser, VerificationToken},
        ports::UserDatabase,
        telemetry::init_test_telemetry,
    };

    #[sqlx::test]
    async fn reject_duplicate_email(pool: Pool<MySql>) {
        init_test_telemetry();
        let mut db = MySqlDatabase { pool };
        let user = NewUser {
            name: Default::default(),
            email: "example@domain.com".into(),
            password_hash: Secret::new(Default::default()),
            verification_token: Secret::new(VerificationToken::new()),
            verified: Default::default(),
            refresh_token: Secret::new(Default::default()),
        };
        let res = db.create_user(&user).await;
        assert!(res.is_ok());
        let res = db.create_user(&user).await;
        assert!(res.is_err_and(|e| matches!(e, Error::EmailTaken)));
    }
}

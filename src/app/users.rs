use crate::{
    domain::{
        error::Result,
        password::{Password, PasswordHash},
        token::Token,
        user::{NewUser, NewUserRequest},
    },
    ports::database::Database,
};

use super::App;

impl<DB> App<DB>
where
    DB: Database,
{
    //TODO tests
    #[tracing::instrument(skip(self))]
    pub async fn signup(&mut self, req: NewUserRequest) -> Result<()> {
        let _: Password = req.password.try_into()?;
        let user = NewUser {
            email: req.email.try_into()?,
            //TODO real hasher
            password_hash: PasswordHash::new("asdf".into()),
            verification_token: Token::generate(),
            verified: false,
            refresh_token: Token::generate(),
        };
        self.database.create_user(&user).await
    }
}

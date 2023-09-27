use secrecy::{ExposeSecret, SecretString};

use crate::telemetry;

use super::{error::Error, secret::Secret};

pub struct Password(SecretString);

pub type PasswordHash = Secret<String>;

impl ExposeSecret<String> for Password {
    fn expose_secret(&self) -> &String {
        self.0.expose_secret()
    }
}

//TODO tests
impl TryFrom<SecretString> for Password {
    type Error = Error;

    fn try_from(value: SecretString) -> Result<Self, Self::Error> {
        validate_password(value.expose_secret().as_str())
            .map(|_| Self(value))
            .map_err(Error::Validation)
            .map_err(telemetry::warn)
    }
}

fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() < 8 {
        Err("password must be at least 8 characters long")?;
    }
    if !password.chars().any(char::is_lowercase) {
        Err("password must contain at least one lowercase character")?;
    }
    if !password.chars().any(char::is_uppercase) {
        Err("password must contain at least one uppercase character")?;
    }
    if !password.chars().any(char::is_numeric) {
        Err("password must contain at least one number")?;
    }
    Ok(())
}

#[cfg(test)]
pub struct FakePassword;

#[cfg(test)]
impl fake::Dummy<FakePassword> for secrecy::SecretString {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(
        _: &FakePassword,
        _: &mut R,
    ) -> Self {
        use fake::{faker::internet::en::Password, Fake};
        Password(8..32 + 1).fake::<String>().into()
    }
}

#[cfg(test)]
impl fake::Dummy<fake::Faker> for PasswordHash {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(
        _: &fake::Faker,
        _: &mut R,
    ) -> Self {
        use fake::Fake;
        Self::new((60..64 + 1).fake())
    }
}

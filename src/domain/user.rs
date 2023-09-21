use secrecy::Secret;

use super::verification_token::VerificationToken;

#[derive(Clone, Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: Secret<String>,
    pub verification_token: VerificationToken,
    pub verified: bool,
    pub refresh_token: Secret<String>,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password_hash: Secret<String>,
    pub verification_token: VerificationToken,
    pub verified: bool,
    pub refresh_token: Secret<String>,
}

#[cfg(test)]
mod fake {
    use fake::{
        faker::{internet::en::SafeEmail, name::en::Name},
        Dummy, Fake, Faker,
    };
    use secrecy::Secret;

    use super::{NewUser, VerificationToken};

    impl Dummy<Faker> for NewUser {
        fn dummy_with_rng<R: fake::Rng + ?Sized>(
            _: &Faker,
            rng: &mut R,
        ) -> Self {
            Self {
                name: Name().fake_with_rng(rng),
                email: SafeEmail().fake_with_rng(rng),
                password_hash: Secret::new(70.fake_with_rng(rng)),
                verification_token: VerificationToken::generate(),
                verified: false,
                refresh_token: Secret::new(32.fake_with_rng(rng)),
            }
        }
    }
}

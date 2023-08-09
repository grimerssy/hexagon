use secrecy::Secret;

use super::VerificationToken;

#[derive(Clone, Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: Secret<String>,
    pub verification_token: Secret<VerificationToken>,
    pub verified: bool,
    pub refresh_token: Secret<String>,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password_hash: Secret<String>,
    pub verification_token: Secret<VerificationToken>,
    pub verified: bool,
    pub refresh_token: Secret<String>,
}

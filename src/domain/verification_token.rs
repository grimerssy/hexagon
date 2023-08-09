use std::fmt;

use secrecy::{zeroize::DefaultIsZeroes, CloneableSecret, DebugSecret};
use uuid::Uuid;

#[derive(Clone, Copy, Default)]
pub struct VerificationToken(Uuid);

impl VerificationToken {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn uuid(&self) -> &Uuid {
        &self.0
    }
}

impl DefaultIsZeroes for VerificationToken {}
impl CloneableSecret for VerificationToken {}
impl DebugSecret for VerificationToken {}

impl fmt::Display for VerificationToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uuid())
    }
}

use std::fmt;

use secrecy::{
    zeroize::DefaultIsZeroes, CloneableSecret, DebugSecret, Zeroize,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct VerificationToken(ZeroizableUuid);

#[derive(Clone, Copy, Default)]
struct ZeroizableUuid(Uuid);

impl VerificationToken {
    pub fn generate() -> Self {
        Self(ZeroizableUuid(Uuid::new_v4()))
    }
}

impl DefaultIsZeroes for ZeroizableUuid {}

impl CloneableSecret for VerificationToken {}
impl DebugSecret for VerificationToken {}

impl Zeroize for VerificationToken {
    fn zeroize(&mut self) {
        self.0.zeroize()
    }
}

impl fmt::Display for VerificationToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 .0)
    }
}

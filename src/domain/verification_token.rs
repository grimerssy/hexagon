use std::fmt;

use secrecy::{
    zeroize::DefaultIsZeroes, CloneableSecret, DebugSecret, ExposeSecret,
    Secret,
};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    mysql::{MySqlTypeInfo, MySqlValueRef},
    Decode, Encode, MySql, Type,
};
use uuid::{fmt::Hyphenated, Uuid};

#[derive(Clone)]
pub struct VerificationToken(Secret<ZeroizableUuid>);

#[derive(Clone, Copy, Default)]
struct ZeroizableUuid(Uuid);

impl VerificationToken {
    #[allow(unused)]
    pub fn generate() -> Self {
        Self(Secret::new(ZeroizableUuid(Uuid::new_v4())))
    }
}

impl CloneableSecret for ZeroizableUuid {}
impl DefaultIsZeroes for ZeroizableUuid {}

impl DebugSecret for VerificationToken {}

impl fmt::Debug for VerificationToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Self::debug_secret(f)
    }
}

impl Type<MySql> for VerificationToken {
    fn type_info() -> MySqlTypeInfo {
        <Hyphenated as Type<MySql>>::type_info()
    }
}

impl Encode<'_, MySql> for VerificationToken {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        self.0.expose_secret().0.hyphenated().encode_by_ref(buf)
    }
}

impl Decode<'_, MySql> for VerificationToken {
    fn decode(value: MySqlValueRef) -> Result<Self, BoxDynError> {
        let uuid = Hyphenated::decode(value)?.into_uuid();
        Ok(Self(Secret::new(ZeroizableUuid(uuid))))
    }
}

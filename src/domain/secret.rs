use std::fmt;

use secrecy::{
    zeroize::DefaultIsZeroes, CloneableSecret, DebugSecret, ExposeSecret,
    Zeroize,
};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    mysql::{MySqlTypeInfo, MySqlValueRef},
    Decode, Encode, MySql, Type,
};

pub struct Secret<T: Zeroize>(secrecy::Secret<T>);

#[derive(Clone, Copy, Default, sqlx::Type)]
#[sqlx(transparent)]
pub struct Zeroizable<T: Clone + Copy + Default>(pub T);

impl<T: Zeroize> Secret<T> {
    pub fn new(value: T) -> Self {
        Self(secrecy::Secret::new(value))
    }
}

impl<T: Zeroize + CloneableSecret> Clone for Secret<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: DebugSecret + Zeroize> fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Zeroize> ExposeSecret<T> for Secret<T> {
    fn expose_secret(&self) -> &T {
        self.0.expose_secret()
    }
}

impl<T: Zeroize + Type<MySql>> Type<MySql> for Secret<T> {
    fn type_info() -> MySqlTypeInfo {
        <T as Type<MySql>>::type_info()
    }
}

impl<'q, T: Zeroize + Encode<'q, MySql>> Encode<'q, MySql> for Secret<T> {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        self.0.expose_secret().encode_by_ref(buf)
    }
}

impl<'r, T: Zeroize + Decode<'r, MySql>> Decode<'r, MySql> for Secret<T> {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(Self(secrecy::Secret::new(T::decode(value)?)))
    }
}

impl<T: Clone + Copy + Default> CloneableSecret for Zeroizable<T> {}
impl<T: Clone + Copy + Default> DefaultIsZeroes for Zeroizable<T> {}
impl<T: Clone + Copy + Default> DebugSecret for Zeroizable<T> {}

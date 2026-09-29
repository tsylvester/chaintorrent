use core::convert::Infallible;
use zeroize::Zeroize;

pub struct Secret<T: Zeroize> {
    pub(super) value: T,
}

pub struct SecretConstructorParams<T: Zeroize> {
    pub value: T,
}

pub type SecretTryNewReturn<T> = Result<Secret<T>, Infallible>;

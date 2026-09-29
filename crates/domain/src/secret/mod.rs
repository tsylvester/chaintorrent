mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{Secret, SecretConstructorParams, SecretTryNewReturn};
use zeroize::Zeroize;

impl<T: Zeroize> Secret<T> {
    pub fn try_new(params: SecretConstructorParams<T>) -> SecretTryNewReturn<T> {
        Ok(Secret {
            value: params.value,
        })
    }

    pub fn expose(&self) -> &T {
        &self.value
    }
}

impl<T: Zeroize> Drop for Secret<T> {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}

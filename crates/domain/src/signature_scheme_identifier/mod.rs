mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    SignatureSchemeIdentifier, SignatureSchemeIdentifierConstructorParams,
    SignatureSchemeIdentifierTryNewErrorReturn, SignatureSchemeIdentifierTryNewReturn,
};

impl SignatureSchemeIdentifier {
    pub fn try_new(
        params: SignatureSchemeIdentifierConstructorParams,
    ) -> SignatureSchemeIdentifierTryNewReturn {
        if params.identifier == 0 {
            return Err(SignatureSchemeIdentifierTryNewErrorReturn::ZeroIdentifier);
        }
        Ok(SignatureSchemeIdentifier {
            identifier: params.identifier,
        })
    }

    pub fn identifier(&self) -> u8 {
        self.identifier
    }
}

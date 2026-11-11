mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    SUITE_IDENTIFIER_LENGTH, SuiteIdentifier, SuiteIdentifierConstructorParams,
    SuiteIdentifierTryNewErrorReturn, SuiteIdentifierTryNewReturn,
};

impl SuiteIdentifier {
    pub fn try_new(params: SuiteIdentifierConstructorParams) -> SuiteIdentifierTryNewReturn {
        if params.identifier.iter().all(|byte| *byte == 0) {
            return Err(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier);
        }
        if params.version == 0 {
            return Err(SuiteIdentifierTryNewErrorReturn::ZeroVersion);
        }
        Ok(SuiteIdentifier {
            identifier: params.identifier,
            version: params.version,
        })
    }

    pub fn identifier(&self) -> &[u8; SUITE_IDENTIFIER_LENGTH] {
        &self.identifier
    }

    pub fn version(&self) -> u16 {
        self.version
    }
}

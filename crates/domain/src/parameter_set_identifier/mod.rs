mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    PARAMETER_SET_IDENTIFIER_LENGTH, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParams, ParameterSetIdentifierTryNewErrorReturn,
    ParameterSetIdentifierTryNewReturn,
};

impl ParameterSetIdentifier {
    pub fn try_new(
        params: ParameterSetIdentifierConstructorParams,
    ) -> ParameterSetIdentifierTryNewReturn {
        if params.bytes.iter().all(|byte| *byte == 0) {
            return Err(ParameterSetIdentifierTryNewErrorReturn::AllZero);
        }
        Ok(ParameterSetIdentifier {
            bytes: params.bytes,
        })
    }

    pub fn as_bytes(&self) -> &[u8; PARAMETER_SET_IDENTIFIER_LENGTH] {
        &self.bytes
    }
}

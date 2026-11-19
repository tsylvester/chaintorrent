mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    HANDSHAKE_KEY_COMMITMENT_LENGTH, HandshakeKeyCommitment,
    HandshakeKeyCommitmentConstructorParams, HandshakeKeyCommitmentTryNewErrorReturn,
    HandshakeKeyCommitmentTryNewReturn,
};

use crate::signature_scheme_identifier::provides::SignatureSchemeIdentifier;

impl HandshakeKeyCommitment {
    pub fn try_new(
        params: HandshakeKeyCommitmentConstructorParams,
    ) -> HandshakeKeyCommitmentTryNewReturn {
        if params.bytes.iter().all(|byte| *byte == 0) {
            return Err(HandshakeKeyCommitmentTryNewErrorReturn::AllZero);
        }
        Ok(HandshakeKeyCommitment {
            bytes: params.bytes,
            scheme: params.scheme,
        })
    }

    pub fn as_bytes(&self) -> &[u8; HANDSHAKE_KEY_COMMITMENT_LENGTH] {
        &self.bytes
    }

    pub fn scheme(&self) -> SignatureSchemeIdentifier {
        self.scheme
    }
}

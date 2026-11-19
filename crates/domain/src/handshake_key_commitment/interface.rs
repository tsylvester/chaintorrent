use crate::signature_scheme_identifier::provides::SignatureSchemeIdentifier;

pub const HANDSHAKE_KEY_COMMITMENT_LENGTH: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandshakeKeyCommitment {
    pub(super) bytes: [u8; HANDSHAKE_KEY_COMMITMENT_LENGTH],
    pub(super) scheme: SignatureSchemeIdentifier,
}

pub struct HandshakeKeyCommitmentConstructorParams {
    pub bytes: [u8; HANDSHAKE_KEY_COMMITMENT_LENGTH],
    pub scheme: SignatureSchemeIdentifier,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HandshakeKeyCommitmentTryNewErrorReturn {
    AllZero,
}

pub type HandshakeKeyCommitmentTryNewReturn =
    Result<HandshakeKeyCommitment, HandshakeKeyCommitmentTryNewErrorReturn>;

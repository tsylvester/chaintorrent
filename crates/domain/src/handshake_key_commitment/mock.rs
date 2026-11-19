#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{
    HANDSHAKE_KEY_COMMITMENT_LENGTH, HandshakeKeyCommitment,
    HandshakeKeyCommitmentConstructorParams,
};
use crate::signature_scheme_identifier::provides::{
    SignatureSchemeIdentifier, build_signature_scheme_identifier,
};

#[derive(Default)]
pub struct HandshakeKeyCommitmentConstructorParamsOverrides {
    pub bytes: Option<[u8; HANDSHAKE_KEY_COMMITMENT_LENGTH]>,
    pub scheme: Option<SignatureSchemeIdentifier>,
}

pub fn build_handshake_key_commitment_constructor_params(
    overrides: HandshakeKeyCommitmentConstructorParamsOverrides,
) -> HandshakeKeyCommitmentConstructorParams {
    HandshakeKeyCommitmentConstructorParams {
        bytes: overrides
            .bytes
            .unwrap_or([0x44; HANDSHAKE_KEY_COMMITMENT_LENGTH]),
        scheme: overrides
            .scheme
            .unwrap_or_else(|| build_signature_scheme_identifier(Default::default())),
    }
}

pub fn build_handshake_key_commitment(
    overrides: HandshakeKeyCommitmentConstructorParamsOverrides,
) -> HandshakeKeyCommitment {
    HandshakeKeyCommitment::try_new(build_handshake_key_commitment_constructor_params(overrides))
        .expect("built handshake key commitment constructor params are admitted")
}

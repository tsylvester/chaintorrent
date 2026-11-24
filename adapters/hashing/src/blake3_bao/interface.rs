use crate::factory::provides::{
    COMMITMENT_INTERFACE_VERSION, CommitmentConcrete, CommitmentDeclaration,
    CommitmentSchemeIdentifier,
};
use core::convert::Infallible;

pub struct Blake3BaoCommitment;

pub struct Blake3BaoCommitmentConstructorParams;

pub type Blake3BaoCommitmentTryNewReturn = Result<Blake3BaoCommitment, Infallible>;

pub const BLAKE3_BAO_CHUNK_LENGTH: u64 = 1024;
pub const BLAKE3_BAO_KEY_LENGTH: usize = 32;

#[derive(Debug, PartialEq, Eq)]
pub enum Blake3BaoCommitmentErrorReturn {
    ContentLengthUnrepresentable { length: usize },
    OffsetUnrepresentable { offset: u64 },
}

impl Blake3BaoCommitment {
    pub const DECLARATION: CommitmentDeclaration = CommitmentDeclaration {
        concrete: CommitmentConcrete::Blake3Bao,
        identifier: CommitmentSchemeIdentifier::Blake3BaoV1,
        chunk_length: BLAKE3_BAO_CHUNK_LENGTH,
        key_length: BLAKE3_BAO_KEY_LENGTH,
        adapter_version: 1,
        interface_version: COMMITMENT_INTERFACE_VERSION,
    };
}

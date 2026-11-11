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

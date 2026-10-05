use domain::{DerivationContext, Secret};
use encoding::{EncodeErrorReturn, IEncoderAdapter};
use kdf::{DeriveKeyErrorReturn, IKeyDerivationAdapter};
use kem::EncapsulatedValue;

pub const PIECE_GROUP_KEY_LENGTH: usize = 32;

pub struct PieceGroupKey {
    pub(super) key: Secret<[u8; PIECE_GROUP_KEY_LENGTH]>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PieceGroupKeyErrorReturn {
    WrongLength { expected: usize, actual: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WrappedPieceGroupKey {
    pub(super) bytes: [u8; PIECE_GROUP_KEY_LENGTH],
}

#[derive(Debug, PartialEq, Eq)]
pub enum WrappedPieceGroupKeyErrorReturn {
    WrongLength { expected: usize, actual: usize },
}

pub struct WrapPieceGroupKeyDeps<'a, E: IEncoderAdapter> {
    pub kdf: &'a dyn IKeyDerivationAdapter,
    pub encoder: &'a E,
}

pub struct WrapPieceGroupKeyParams;

pub struct WrapPieceGroupKeyPayload<'a> {
    pub encapsulated: &'a EncapsulatedValue,
    pub context: &'a DerivationContext,
    pub piece_group_key: &'a PieceGroupKey,
}

pub struct WrapPieceGroupKeySuccessReturn {
    pub wrapped: WrappedPieceGroupKey,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WrapPieceGroupKeyErrorReturn {
    Encoding(EncodeErrorReturn),
    KeyDerivation(DeriveKeyErrorReturn),
    WrappingKeyLength { expected: usize, actual: usize },
}

pub type WrapPieceGroupKeyReturn =
    Result<WrapPieceGroupKeySuccessReturn, WrapPieceGroupKeyErrorReturn>;

pub type WrapPieceGroupKeyFn<E> = fn(
    &WrapPieceGroupKeyDeps<'_, E>,
    WrapPieceGroupKeyParams,
    WrapPieceGroupKeyPayload<'_>,
) -> WrapPieceGroupKeyReturn;
